use super::{CodeMirror, RenderedValue, ValueOrError};
use crate::bindings::create_split;
use crate::fs_worker::{FsRequest, FsResponse, FsWorker};
use crate::{URL_PREFIX, VERSION};
use ev::Event;
use futures::stream::StreamExt;
use gloo_worker::{Spawnable, WorkerBridge};
use html::Div;
use leptos::*;
use leptos_dom::helpers::TimeoutHandle;
use leptos_router::*;
use pinned::mpsc;
use pinned::mpsc::UnboundedReceiver;
use scamper_rs::{interpreter::Output, Engine};
use std::{cell::RefCell, rc::Rc, time::Duration};
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

#[cfg(debug_assertions)]
use leptos::SpecialNonReactiveZone;

#[component]
pub fn Ide() -> impl IntoView {
    let params = use_params_map();
    let current_file = create_memo(move |_| params.with(|params| params.get("file").cloned()));

    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal::<Option<String>>(None);

    let (last_code, set_last_code) = create_signal(String::new());
    let (timer_handle, set_timer_handle) = create_signal::<Option<TimeoutHandle>>(None);

    let (start_input, set_start_input) = create_signal(None);

    let (dirty, set_dirty) = create_signal(false);
    let (input, set_input) = create_signal(String::new());
    let (output, set_output) = create_signal(Vec::new());

    let (worker_bridge, set_worker_bridge) = create_signal(None);
    let (receiver, set_receiver) = create_signal(None);

    // load fs worker and initial file content
    create_effect(move |_| {
        let Some(current_file) = current_file.get() else {
            return;
        };

        // create fs worker instance to read and write the file
        if worker_bridge.get().is_none() {
            let mut spawner = FsWorker::spawner();

            let (tx, rx) = mpsc::unbounded();
            spawner.callback(move |output| {
                let _ = tx.send_now(output);
            });
            set_receiver.set(Some(Rc::new(RefCell::new(rx))));

            let bridge = spawner.spawn(&format!("{URL_PREFIX}/fs_worker.js"));

            set_worker_bridge.set(Some(Rc::new(bridge)));
        }

        let Some(bridge) = worker_bridge.get() else {
            return;
        };

        let Some(receiver) = receiver.get() else {
            return;
        };

        // read initial file content
        spawn_local(async move {
            bridge.send(FsRequest::ReadFile(current_file));

            let result = receiver.borrow_mut().next().await.unwrap();

            match result {
                FsResponse::FileContent(text) => {
                    set_input.set(text.clone());
                    set_start_input.set(Some(text));
                    set_loading.set(false);
                }
                FsResponse::Error(e) => {
                    if e == "Failed to get file handle" {
                        set_error.set(Some(String::from(
                            "File already in use by another tab or window",
                        )));
                    } else {
                        set_error.set(Some(e));
                    }
                }
                _ => {
                    set_error.set(Some("Error reading file".to_string()));
                }
            };
        });
    });

    let save = |bridge: Rc<WorkerBridge<FsWorker>>,
                receiver: Rc<RefCell<UnboundedReceiver<FsResponse>>>,
                current_file: String,
                code: String,
                set_error: WriteSignal<Option<String>>| {
        spawn_local(async move {
            bridge.send(FsRequest::WriteFile(current_file, code));

            let result = receiver.borrow_mut().next().await.unwrap();

            match result {
                FsResponse::WriteComplete => {}
                FsResponse::Error(e) => {
                    set_error.set(Some(e));
                }
                _ => {
                    set_error.set(Some("Error writing file".to_string()));
                }
            };
        });
    };

    // debounced save effect
    create_effect(move |_| {
        let code = input.get();

        let Some(current_file) = current_file.get() else {
            return;
        };
        let Some(bridge) = worker_bridge.get() else {
            return;
        };
        let Some(receiver) = receiver.get() else {
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
                save(bridge, receiver, current_file, code, set_error);
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

    // save file on beforeunload
    let closure = Closure::wrap(Box::new(move |_: Event| {
        if error.get().is_some() {
            return;
        }

        let Some(current_file) = current_file.get() else {
            return;
        };
        let Some(bridge) = worker_bridge.get() else {
            return;
        };
        let Some(receiver) = receiver.get() else {
            return;
        };
        let code = input.get();
        save(bridge, receiver, current_file, code, set_error);
    }) as Box<dyn FnMut(_)>);

    window()
        .add_event_listener_with_callback("beforeunload", closure.as_ref().unchecked_ref())
        .expect("failed to set beforeunload");

    closure.forget();

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
