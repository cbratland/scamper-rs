use crate::URL_PREFIX;
use futures::StreamExt;
use gloo_worker::Spawnable;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{global, Reflect};
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemGetFileOptions,
    FileSystemSyncAccessHandle,
};

#[derive(Serialize, Deserialize)]
pub enum FsRequest {
    ReadFile(String),          // path
    WriteFile(String, String), // (path, content)
    MoveFile(String, String),  // (source, destination)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FsResponse {
    FileContent(String),
    WriteComplete,
    MoveComplete,
    Error(String),
}

pub enum FsMessage {
    DirectoryHandle(Option<FileSystemDirectoryHandle>),
    FileHandle(Option<FileSystemSyncAccessHandle>),
}

#[derive(Debug, Clone)]
pub struct FsWorker {
    root_dir: Option<FileSystemDirectoryHandle>,
    curr_file: Option<FileSystemSyncAccessHandle>,
}

impl Worker for FsWorker {
    type Input = FsRequest;
    type Output = FsResponse;
    type Message = FsMessage;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            root_dir: None,
            curr_file: None,
        }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, msg: Self::Message) {
        match msg {
            FsMessage::DirectoryHandle(dir) => self.root_dir = dir,
            FsMessage::FileHandle(file) => self.curr_file = file,
        }
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        let needs_dir_handle = self.root_dir.is_none();
        let needs_file_handle =
            self.curr_file.is_none() && !matches!(msg, FsRequest::MoveFile(_, _));

        // initialize handles if not already done
        if needs_dir_handle || needs_file_handle {
            let dir_handle = self.root_dir.clone();
            let file_handle = self.curr_file.clone();
            wasm_bindgen_futures::spawn_local({
                let scope = scope.clone();
                let id = id.clone();
                async move {
                    let new_dir_handle = if needs_dir_handle {
                        let Ok(navigator) =
                            Reflect::get(&global(), &JsValue::from_str("navigator"))
                                .and_then(|v| v.dyn_into::<web_sys::WorkerNavigator>())
                        else {
                            scope.respond(id, FsResponse::Error("Failed to get navigator".into()));
                            return;
                        };
                        let storage = navigator.storage();

                        match JsFuture::from(storage.get_directory()).await {
                            Ok(handle) => {
                                let dir_handle: FileSystemDirectoryHandle = handle.unchecked_into();

                                scope.send_message(FsMessage::DirectoryHandle(Some(
                                    dir_handle.clone(),
                                )));

                                Some(dir_handle)
                            }
                            Err(_) => {
                                scope.respond(
                                    id,
                                    FsResponse::Error("Failed to get root directory".into()),
                                );
                                return;
                            }
                        }
                    } else {
                        dir_handle
                    };

                    let Some(dir_handle) = new_dir_handle else {
                        scope.respond(id, FsResponse::Error("Failed to get root directory".into()));
                        return;
                    };

                    let file_handle = if needs_file_handle {
                        let path = match &msg {
                            FsRequest::ReadFile(path) => path,
                            FsRequest::WriteFile(path, _) => &path.clone(),
                            FsRequest::MoveFile(_, _) => unreachable!(),
                        };
                        let file_handle = Self::get_file_handle(&dir_handle, &path).await.ok();
                        scope.send_message(FsMessage::FileHandle(file_handle.clone()));
                        file_handle
                    } else {
                        file_handle
                    };

                    Self::handle_request(scope, id, msg, dir_handle, file_handle);
                }
            });
        } else {
            let Some(root_dir) = self.root_dir.clone() else {
                scope.respond(id, FsResponse::Error("Failed to get root directory".into()));
                return;
            };

            Self::handle_request(scope.clone(), id, msg, root_dir, self.curr_file.clone());
        }
    }

    fn destroy(
        &mut self,
        _scope: &WorkerScope<Self>,
        destruct: gloo_worker::WorkerDestroyHandle<Self>,
    ) {
        let _destruct = destruct;
        if let Some(file) = self.curr_file.take() {
            file.close();
        }
    }
}

impl FsWorker {
    fn handle_request(
        scope: WorkerScope<Self>,
        id: HandlerId,
        msg: FsRequest,
        root_dir: FileSystemDirectoryHandle,
        file_handle: Option<FileSystemSyncAccessHandle>,
    ) {
        let scope = scope.clone();
        wasm_bindgen_futures::spawn_local(async move {
            match msg {
                FsRequest::ReadFile(_) => {
                    let Some(file_handle) = file_handle else {
                        scope.respond(id, FsResponse::Error("Failed to get file handle".into()));
                        return;
                    };
                    match Self::read_file(&file_handle).await {
                        Ok(content) => scope.respond(id, FsResponse::FileContent(content)),
                        Err(e) => scope.respond(id, FsResponse::Error(e)),
                    }
                }
                FsRequest::WriteFile(_, content) => {
                    let Some(file_handle) = file_handle else {
                        scope.respond(id, FsResponse::Error("Failed to get file handle".into()));
                        return;
                    };
                    match Self::write_file(&file_handle, &content).await {
                        Ok(()) => scope.respond(id, FsResponse::WriteComplete),
                        Err(e) => scope.respond(id, FsResponse::Error(e)),
                    }
                }
                FsRequest::MoveFile(old_path, new_path) => {
                    match Self::move_file(&root_dir, &old_path, &new_path).await {
                        Ok(()) => scope.respond(id, FsResponse::MoveComplete),
                        Err(e) => scope.respond(id, FsResponse::Error(e)),
                    }
                }
            }
        });
    }

    async fn get_file_handle(
        root_dir: &FileSystemDirectoryHandle,
        path: &str,
    ) -> Result<FileSystemSyncAccessHandle, String> {
        let file_handle = JsFuture::from(root_dir.get_file_handle(path))
            .await
            .map_err(|_| "Failed to get file handle".to_string())?;

        let sync_handle = JsFuture::from(
            file_handle
                .dyn_ref::<web_sys::FileSystemFileHandle>()
                .ok_or("Invalid file handle")?
                .create_sync_access_handle(),
        )
        .await
        .map_err(|_| "Failed to create sync handle".to_string())?;

        Ok(sync_handle.unchecked_into())
    }

    async fn read_file(handle: &FileSystemSyncAccessHandle) -> Result<String, String> {
        let size = handle
            .get_size()
            .map_err(|_| "Failed to get file size".to_string())? as usize;
        let mut buffer = vec![0u8; size];
        handle
            .read_with_u8_array(&mut buffer)
            .map_err(|_| "Failed to read file".to_string())?;
        String::from_utf8(buffer).map_err(|_| "Invalid UTF-8".to_string())
    }

    async fn write_file(handle: &FileSystemSyncAccessHandle, content: &str) -> Result<(), String> {
        let content_bytes = content.as_bytes();
        handle
            .truncate_with_f64(0.0) // content_bytes.len() as f64
            .map_err(|_| "Failed to clear file".to_string())?;
        handle
            .write_with_u8_array(content_bytes)
            .map_err(|_| "Failed to write file".to_string())?;
        handle
            .flush()
            .map_err(|_| "Failed to save file".to_string())?;

        Ok(())
    }

    async fn move_file(
        root_dir: &FileSystemDirectoryHandle,
        old_path: &str,
        new_path: &str,
    ) -> Result<(), String> {
        // make sure file isn't being accessed by another worker
        let old_handle: FileSystemFileHandle = JsFuture::from(root_dir.get_file_handle(old_path))
            .await
            .map_err(|e| format!("Failed to get file handle: {:?}", e))?
            .unchecked_into();
        let sync_handle: FileSystemSyncAccessHandle =
            JsFuture::from(old_handle.create_sync_access_handle())
                .await
                .map_err(|_| "File open in another tab".to_string())?
                .unchecked_into();
        sync_handle.close();

        // read old file contents
        let old_file: web_sys::File = JsFuture::from(old_handle.get_file())
            .await
            .map_err(|e| format!("Failed to get file: {:?}", e))?
            .unchecked_into();
        let contents = JsFuture::from(old_file.text())
            .await
            .map_err(|e| format!("Failed to get file contents: {:?}", e))?
            .as_string()
            .ok_or("Failed to get file contents")?;

        // create new file
        let options = FileSystemGetFileOptions::new();
        options.set_create(true);
        let new_file = JsFuture::from(root_dir.get_file_handle_with_options(new_path, &options))
            .await
            .map_err(|e| format!("Failed to create file: {:?}", e))?;

        // write old file contents to new file
        let new_file_handle: FileSystemSyncAccessHandle = JsFuture::from(
            new_file
                .dyn_ref::<web_sys::FileSystemFileHandle>()
                .ok_or("Invalid file handle")?
                .create_sync_access_handle(),
        )
        .await
        .map_err(|_| "Failed to create sync handle".to_string())?
        .unchecked_into();

        Self::write_file(&new_file_handle, &contents).await?;

        // delete old file
        JsFuture::from(root_dir.remove_entry(old_path))
            .await
            .map_err(|e| format!("Failed to remove file: {:?}", e))?;

        Ok(())
    }
}

// spawn an fs worker and send a single request
pub async fn send_request(request: FsRequest) -> Option<FsResponse> {
    let mut spawner = FsWorker::spawner();

    let (tx, mut rx) = pinned::mpsc::unbounded();
    spawner.callback(move |output| {
        let _ = tx.send_now(output);
    });

    let bridge = spawner.spawn(&format!("{URL_PREFIX}/fs_worker.js"));

    bridge.send(request);

    rx.next().await
}
