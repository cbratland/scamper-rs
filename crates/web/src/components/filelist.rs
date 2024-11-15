use crate::{
    fs_worker::{self, FsRequest, FsResponse},
    URL_PREFIX, VERSION,
};
use leptos::*;
use leptos_meta::Style;
use leptos_router::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemGetFileOptions, FileSystemHandle,
    StorageManager,
};

#[derive(Debug, Clone)]
struct FileEntry {
    name: String,
    preview: Option<String>,
    is_directory: bool,
}

async fn get_file_preview(file_handle: &FileSystemFileHandle) -> Result<String, String> {
    let file = JsFuture::from(file_handle.get_file())
        .await
        .map_err(|e| format!("Failed to get file: {:?}", e))?
        .unchecked_into::<web_sys::File>();

    let text = JsFuture::from(file.text())
        .await
        .map_err(|e| format!("Failed to read file: {:?}", e))?
        .as_string()
        .ok_or("Failed to convert file content to string")?;

    let preview = text.lines().take(5).collect::<Vec<_>>().join("\n");

    Ok(preview)
}

async fn load_directory() -> Result<Vec<FileEntry>, String> {
    let navigator = window().navigator();
    let storage: StorageManager = navigator.storage();

    let directory_handle: FileSystemDirectoryHandle = JsFuture::from(storage.get_directory())
        .await
        .map_err(|e| format!("Failed to get directory: {:?}", e))?
        .unchecked_into();

    let mut file_entries = Vec::new();

    let entries = directory_handle.entries();

    loop {
        let next = JsFuture::from(entries.next().map_err(|_| "Failed to get next entry")?)
            .await
            .map_err(|e| format!("Failed to get next entry: {:?}", e))?;

        let done = web_sys::js_sys::Reflect::get(&next, &JsValue::from_str("done"))
            .map_err(|e| format!("Failed to get done status: {:?}", e))?
            .as_bool()
            .unwrap_or(true);

        if done {
            break;
        }

        let value = web_sys::js_sys::Reflect::get(&next, &JsValue::from_str("value"))
            .map_err(|e| format!("Failed to get value: {:?}", e))?;

        let array: &web_sys::js_sys::Array = value.unchecked_ref();
        let handle: FileSystemHandle = array.get(1).unchecked_into();
        let is_directory = handle.dyn_ref::<FileSystemDirectoryHandle>().is_some();

        let preview = if !is_directory {
            let file_handle: FileSystemFileHandle = handle.clone().unchecked_into();
            match get_file_preview(&file_handle).await {
                Ok(preview) => Some(preview),
                Err(_) => None,
            }
        } else {
            None
        };

        file_entries.push(FileEntry {
            name: handle.name(),
            preview,
            is_directory,
        });
    }

    file_entries.sort_by(|a, b| match (a.is_directory, b.is_directory) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    Ok(file_entries)
}

async fn create_new_file(
    directory_handle: &FileSystemDirectoryHandle,
    filename: &str,
) -> Result<(), String> {
    let options = FileSystemGetFileOptions::new();
    options.set_create(true);
    _ = JsFuture::from(directory_handle.get_file_handle_with_options(filename, &options))
        .await
        .map_err(|e| format!("Failed to create file: {:?}", e))?;

    Ok(())
}

async fn delete_file(
    directory_handle: &FileSystemDirectoryHandle,
    name: &str,
) -> Result<(), String> {
    JsFuture::from(directory_handle.remove_entry(name))
        .await
        .map_err(|e| format!("Failed to remove file: {:?}", e))?;

    Ok(())
}

async fn rename_file(old_name: String, new_name: String) -> Result<(), String> {
    let result = fs_worker::send_request(FsRequest::MoveFile(old_name, new_name)).await;

    match result {
        Some(response) => match response {
            FsResponse::Error(e) => Err(e),
            _ => Ok(()),
        },
        None => Err("Failed to rename file".to_string()),
    }
}

async fn download_file(file_handle: &FileSystemFileHandle) -> Result<(), String> {
    let file = JsFuture::from(file_handle.get_file())
        .await
        .map_err(|e| format!("Failed to get file: {:?}", e))?
        .unchecked_into::<web_sys::File>();

    let url = web_sys::Url::create_object_url_with_blob(&file)
        .map_err(|_| "Failed to create object URL")?;

    let document = window().document().ok_or("No document found")?;

    let anchor = document
        .create_element("a")
        .map_err(|_| "Failed to create anchor element")?;

    anchor
        .set_attribute("href", &url)
        .map_err(|_| "Failed to set href attribute")?;
    anchor
        .set_attribute("download", &file_handle.name())
        .map_err(|_| "Failed to set download attribute")?;

    document
        .body()
        .ok_or("No body found")?
        .append_child(&anchor)
        .map_err(|_| "Failed to append anchor")?;

    if let Some(element) = anchor.dyn_ref::<web_sys::HtmlElement>() {
        element.click();
    }

    document
        .body()
        .ok_or("No body found")?
        .remove_child(&anchor)
        .map_err(|_| "Failed to remove anchor")?;

    web_sys::Url::revoke_object_url(&url).map_err(|_| "Failed to revoke object URL")?;

    Ok(())
}

#[component]
pub fn FileList() -> impl IntoView {
    let (files, set_files) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);

    let (directory_handle, set_directory_handle) =
        create_signal::<Option<FileSystemDirectoryHandle>>(None);

    create_effect(move |_| {
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            let navigator = window().navigator();
            let storage: StorageManager = navigator.storage();

            match JsFuture::from(storage.get_directory()).await {
                Ok(handle) => {
                    let dir_handle: FileSystemDirectoryHandle = handle.unchecked_into();
                    set_directory_handle.set(Some(dir_handle.clone()));

                    match load_directory().await {
                        Ok(file_entries) => {
                            set_files.set(file_entries);
                            set_error.set(None);
                        }
                        Err(e) => {
                            set_error.set(Some(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to get directory handle: {:?}", e)));
                }
            }
            set_loading.set(false);
        });
    });

    let create_new_program = move |_| {
        if let Some(filename) = window().prompt_with_message("Enter file name:").unwrap() {
            // if !filename.ends_with(".scm") {
            //     window()
            //         .alert_with_message("File name must end with .scm")
            //         .unwrap();
            //     return;
            // }

            if let Some(dir_handle) = directory_handle.get() {
                set_loading.set(true);

                spawn_local(async move {
                    match create_new_file(&dir_handle, &filename).await {
                        Ok(_) => match load_directory().await {
                            Ok(file_entries) => {
                                set_files.set(file_entries);
                                set_error.set(None);
                            }
                            Err(e) => {
                                set_error.set(Some(e.to_string()));
                            }
                        },
                        Err(e) => {
                            set_error.set(Some(e.to_string()));
                        }
                    }
                    set_loading.set(false);
                });
            }
        }
    };

    view! {
        // <Stylesheet href="file-picker.css"/>
        <Style>
        "html,
        body {
			width: 100%;
			height: 100%;
			margin: 0px;
			padding: 0px;
			font-family:
				-apple-system,
				BlinkMacSystemFont,
				avenir next,
				avenir,
				segoe ui,
				helvetica neue,
				helvetica,
				Cantarell,
				Ubuntu,
				roboto,
				noto,
				arial,
				sans-serif;
			font-size: 1em;
        }

        #ide {
			height: 100%;
			display: flex;
			flex-direction: column;
        }

        #loading {
			display: block; /* Hidden by default */
			position: fixed; /* Stay in place */
			z-index: 1; /* Sit on top */
			padding-top: 100px; /* Location of the box */
			left: 0;
			top: 0;
			width: 100%; /* Full width */
			height: 100%; /* Full height */
			overflow: auto; /* Enable scroll if needed */
			background-color: rgb(0, 0, 0); /* Fallback color */
			background-color: rgba(0, 0, 0, 0.4); /* Black w/ opacity */
        }

        #loading-content {
			background-color: #fefefe;
			margin: auto;
			padding: 20px;
			border: 1px solid #888;
			width: 80%;
        }

        #header {
			background: #eee;
			color: #333;
			padding: 0.5em;
			flex: 0 0 auto;

			display: flex;
			flex-direction: row;
			flex-wrap: wrap;
			justify-content: space-between;
        }

        .file {
			flex: 0 0 auto;
			width: 200px;
			height: 200px;
			margin: 10px;
			border: 1px solid black;

			text-decoration: none;
			color: black;

			display: flex;
			flex-flow: column wrap;
			align-items: center;
			justify-content: center;
        }

        .file:hover {
        	background-color: #ddffdd;
        }

        .file .actions {
        	visibility: hidden;
        }

        .file:hover .actions {
        	visibility: visible;
        }

        .file .preview {
			width: 200px;
			height: 100px;
			border: 1px dotted blue;
			font-family: monospace;
			font-size: 1em;
			white-space: pre;
			overflow: hidden;
        }

        #content {
			background-color: White;
			color: Black;
			display: flex;
			display: flex;
			flex-flow: row wrap;
			align-content: start;
			width: 100%;
			flex: 1;
			min-height: 0;
		}"
        </Style>

        <div id="ide">
            <div id="header">
                <div class="text-align: left;">
                    "scamper-rs " <span id="version">{format!("({})", VERSION)}</span> " ⋅ "
                    <a href=format!("{URL_PREFIX}/docs") target="_BLANK">"Docs"</a> // " ⋅ "
                    // <a href="reference.html">Reference</a>

                    {move || if loading.get() {
                        view! { " ⋅ " <span>"Loading..."</span> }.into_view()
                    } else {
                        view! { }.into_view()
                    }}
                </div>
                <div class="text-align: right; font-size: 0.75em; color: #333;">
                    <a href="https://github.com/cbratland/scamper-rs"><i class="fa-brands fa-github"></i></a> " ⋅ "
                    <em><a href="https://github.com/cbratland/scamper-rs/issues">Report an issue</a></em>
                </div>
            </div>
            <div id="content">
                {move || error.get().map(|err| view! {
                    <div class="error">{err}</div>
                })}

                <For
                    each=move || files.get().into_iter().filter(|f| !f.is_directory)
                    key=|file| file.name.clone()
                    children=move |file| {
                        let name = file.name.clone();

                        view! {
                            <div class="file" on:click=move |_| {
                                use_navigate()(&format!("/file/{}", name), Default::default());
                            }>
                                <div class="header">{file.name.clone()}</div>
                                <div class="preview">{file.preview.unwrap_or_default()}</div>
                                <div class="last-modified"></div>
                                <div class="actions">
                                    {
                                        let name = file.name.clone();
                                        let name2 = file.name.clone();
                                        view! {
                                            <button
                                                class="fa-solid fa-download"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                    if let Some(dir_handle) = directory_handle.get() {
                                                        let name = name.clone();
                                                        spawn_local(async move {
                                                            set_loading.set(true);
                                                            match JsFuture::from(dir_handle.get_file_handle(&name)).await {
                                                                Ok(handle) => {
                                                                    let file_handle: FileSystemFileHandle = handle.unchecked_into();
                                                                    if let Err(err) = download_file(&file_handle).await {
                                                                        set_error.set(Some(err));
                                                                    }
                                                                }
                                                                Err(e) => {
                                                                    set_error.set(Some(format!("Failed to get file handle: {:?}", e)));
                                                                }
                                                            }
                                                            set_loading.set(false);
                                                        });
                                                    }
                                                }
                                            />
                                            <button
                                                class="fa-solid fa-pencil"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                    if let Some(new_name) = window().prompt_with_message("Enter new file name:").unwrap() {
                                                        let name = file.name.clone();
                                                        spawn_local(async move {
                                                            set_loading.set(true);
                                                            match rename_file(name.clone(), new_name.clone()).await {
                                                                Ok(_) => {
                                                                    // update file name in the list
                                                                    set_files.set(files.get().into_iter().map(|entry| {
                                                                        if entry.name == name {
                                                                            FileEntry {
                                                                                name: new_name.clone(),
                                                                                ..entry
                                                                            }
                                                                        } else {
                                                                            entry
                                                                        }
                                                                    }).collect());
                                                                },
                                                                Err(err) => {
                                                                    set_error.set(Some(err.to_string()));
                                                                }
                                                            }
                                                            set_loading.set(false);
                                                        });
                                                    }
                                                }
                                            />
                                            <button
                                                class="fa-solid fa-trash"
                                                on:click=move |e| {
                                                    e.stop_propagation();
                                                    if let Some(dir_handle) = directory_handle.get() {
                                                        let name = name2.clone();
                                                        spawn_local(async move {
                                                            set_loading.set(true);
                                                            match delete_file(&dir_handle, &name).await {
                                                                Ok(_) => {
                                                                    // remove file from the list
                                                                    set_files.set(files.get().into_iter().filter(|entry| entry.name != name).collect());
                                                                },
                                                                Err(err) => {
                                                                   set_error.set(Some(err.to_string()));
                                                                }
                                                            }
                                                            set_loading.set(false);
                                                        });
                                                    }
                                                }
                                            />
                                        }
                                    }
                                </div>
                            </div>
                        }
                    }
                />

                <div class="file" on:click=create_new_program>
                    <div>"Create a new program"</div>
                </div>
            </div>
        </div>
    }
}
