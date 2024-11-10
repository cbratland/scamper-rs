use std::time::Duration;

use super::{CodeMirror, RenderedValue, ValueOrError};
use crate::bindings::create_split;
use crate::{URL_PREFIX, VERSION};
use html::Div;
use leptos::*;
use leptos_dom::helpers::TimeoutHandle;
use leptos_router::*;
use scamper_rs::{interpreter::Output, Engine};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    FileSystemDirectoryHandle, FileSystemFileHandle, FileSystemWritableFileStream, HtmlElement,
    StorageManager,
};

#[cfg(debug_assertions)]
use leptos::SpecialNonReactiveZone;

#[component]
pub fn Ide() -> impl IntoView {
    let params = use_params_map();
    let current_file = create_memo(move |_| params.with(|params| params.get("file").cloned()));

    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(Option::<String>::None);

    let (last_code, set_last_code) = create_signal(String::new());
    let (timer_handle, set_timer_handle) = create_signal(None::<TimeoutHandle>);

    let (file_handle, set_file_handle) = create_signal(None);
    let (start_input, set_start_input) = create_signal(None);

    let (dirty, set_dirty) = create_signal(false);
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(Vec::new());

    // load file for starting input
    create_effect(move |_| {
        let Some(current_file) = current_file.get() else {
            return;
        };

        spawn_local(async move {
            let navigator = window().navigator();
            let storage: StorageManager = navigator.storage();

            // get storage directory
            match JsFuture::from(storage.get_directory()).await {
                Ok(handle) => {
                    let dir_handle: FileSystemDirectoryHandle = handle.unchecked_into();

                    // get current file
                    match JsFuture::from(dir_handle.get_file_handle(&current_file)).await {
                        Ok(file_handle_js) => {
                            let file_handle: FileSystemFileHandle = file_handle_js.unchecked_into();
                            set_file_handle.set(Some(file_handle.clone()));

                            // read file content
                            match JsFuture::from(file_handle.get_file()).await {
                                Ok(file) => {
                                    let file: web_sys::File = file.unchecked_into();
                                    let text = JsFuture::from(file.text())
                                        .await
                                        .ok()
                                        .and_then(|t| t.as_string())
                                        .unwrap_or_default();
                                    set_input.set(text.clone());
                                    set_start_input.set(Some(text.clone()));
                                }
                                Err(_) => {
                                    set_error.set(Some("Error reading file contents".to_string()));
                                }
                            }
                        }
                        Err(_) => {
                            set_error.set(Some("Error reading file".to_string()));
                        }
                    }
                }
                Err(_) => {
                    set_error.set(Some("Error reading browser storage".to_string()));
                }
            }

            set_loading.set(false);
        });
    });

    // debounced save effect
    create_effect(move |_| {
        let code = input.get();

        let Some(file_handle) = file_handle.get() else {
            return;
        };

        if code == last_code.get() {
            return;
        }

        set_last_code.set(code.clone());

        // cancel existing timer
        if let Some(handle) = timer_handle.get() {
            handle.clear();
        }

        // create timer to save code after 500ms
        let handle = set_timeout_with_handle(
            move || {
                spawn_local(async move {
                    let stream: FileSystemWritableFileStream =
                        match JsFuture::from(file_handle.create_writable()).await {
                            Ok(writable) => writable.unchecked_into(),
                            Err(_) => {
                                return;
                            }
                        };

                    if let Ok(promise) = stream.truncate_with_f64(0.0) {
                        let _ = JsFuture::from(promise).await;
                    }

                    if let Ok(promise) = stream.write_with_str(&code) {
                        let _ = JsFuture::from(promise).await;
                    }

                    let _ = JsFuture::from(stream.close()).await;
                });
            },
            Duration::from_millis(500),
        )
        .expect("failed to set timeout");

        set_timer_handle.set(Some(handle));
    });

    // remove timer when component is destroyed
    on_cleanup(move || {
        if let Some(handle) = timer_handle.get() {
            handle.clear();
        }
    });

    let editor = create_node_ref::<Div>();
    let results = create_node_ref::<Div>();

    // create split
    create_effect(move |_| {
        if loading.get() {
            return;
        }
        use wasm_bindgen::JsCast;
        if let (Some(editor), Some(results)) = (
            editor
                .get()
                .and_then(|n| n.dyn_ref::<HtmlElement>().cloned()),
            results
                .get()
                .and_then(|n| n.dyn_ref::<HtmlElement>().cloned()),
        ) {
            create_split(vec![editor, results], vec![65.0, 35.0]);
        }
    });

    // update stored input code when input changes
    let on_change = move |txt: Option<String>| {
        #[cfg(debug_assertions)]
        let prev = SpecialNonReactiveZone::enter();

        set_input.set(txt.clone().unwrap_or_default());
        if !output.get().is_empty() {
            set_dirty.set(true);
        }

        #[cfg(debug_assertions)]
        SpecialNonReactiveZone::exit(prev);
    };

    // handle run button click
    let run_click = move |_| {
        let code = input.get();
        let engine = Engine::new();

        let values = match engine.run(&code) {
            Ok(values) => values
                .into_iter()
                .map(|v| match v {
                    Output::Value(v) => ValueOrError::Value(v),
                    Output::Error(err) => ValueOrError::Error(err.emit_to_string(&code)),
                })
                .collect(),
            Err(err) => vec![ValueOrError::Error(err.emit_to_web_string(&code))],
        };

        set_output.set(values);
        set_dirty.set(false);
    };

    view! {
        <div id="ide">
        <div id="header">
              <div class="text-align: left;">
                <a href=format!("{URL_PREFIX}/")>"scamper-rs"</a>
                " "
                <span id="version">{format!("({})", VERSION)}</span>
                " ⋅ "
                <span id="current-file">{move || current_file.get()}</span>
                " ⋅ "
                <button id="run" class="fa-solid fa-play" on:click=run_click></button>
                // " "
                // <button id="step" class="fa-solid fa-route" disabled></button>
                // " "
                // <button id="run-window" class="fa-solid fa-window-maximize" disabled></button>
                " ⋅ "
                <a href=format!("{URL_PREFIX}/docs") target="_BLANK">"Docs"</a>
                // " ⋅ "
                // <a href="reference.html">"Reference"</a>
              </div>
              <div class="text-align: right; font-size: 0.75em; color: #333;">
                <a href="https://github.com/cbratland/scamper-rs" target="_blank"><i class="fa-brands fa-github"></i></a> " ⋅ "
                <em><a href="https://github.com/cbratland/scamper-rs/issues" target="_blank">"Report an issue"</a></em>
              </div>
            </div>
            <div id="content">
                {move || {
                    if let Some(input) = start_input.get() {
                        view! {
                            <CodeMirror
                                input=input
                                on_change
                                node_ref=editor
                            />
                        }.into_view()
                    } else {
                        view! {
                            <div id="editor"></div>
                        }.into_view()
                    }
                }}

                <div id="results" node_ref=results>
                    <div id="results-toolbar">
                        <div class="text-align: left;">
                            <button
                                id="step-once"
                                class="fa-solid fa-shoe-prints"
                                disabled
                            ></button>
                            " "
                            <button
                                id="step-stmt"
                                class="fa-solid fa-forward-step"
                                disabled
                            ></button>
                            " "
                            <button
                                id="step-all"
                                class="fa-solid fa-forward"
                                disabled
                            ></button>
                        </div>
                        <div
                            id="results-status"
                            class="text-align: right;"
                            style:display={move || if dirty.get() { "block" } else { "none" }}
                        >
                            <em>"(Warning: results out of sync with updated code)"</em>
                        </div>
                    </div>
                    <div id="output">
                        {move || output.get().into_iter().map(|item| {
                            view! {
                                <div class="scamper-output">
                                    <RenderedValue value=item />
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </div>

        <div id="loading" style:display={move || if loading.get() || error.get().is_some() { "block" } else { "none" }}>
            <div id="loading-content">
                {move || {
                    if let Some(err) = error.get() {
                        view! {
                            {err}
                        }.into_view()
                    } else {
                        view! {
                           "Loading Scamper.."
                        }.into_view()
                    }
                }}
            </div>
        </div>
    }
}
