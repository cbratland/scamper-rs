use super::{CodeMirror, RenderedValue, ValueOrError};
use crate::bindings::create_split;
use crate::VERSION;
use html::Div;
use leptos::SpecialNonReactiveZone;
use leptos::*;
use scamper_rs::{interpreter::Output, Engine};
use web_sys::HtmlElement;

const STORAGE_KEY: &str = "scamper_code";

#[component]
pub fn Ide() -> impl IntoView {
    // let (loading, set_loading) = create_signal(true);
    let (dirty, set_dirty) = create_signal(false);

    // load stored input code
    let starting_input = window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
        .unwrap_or_default();
    let (input, set_input) = create_signal(starting_input.clone());
    let output = create_rw_signal(Vec::new());

    let editor = create_node_ref::<Div>();
    let results = create_node_ref::<Div>();

    // create split
    create_effect(move |_| {
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
        let p = SpecialNonReactiveZone::enter();
        set_input.set(txt.clone().unwrap_or_default());
        if !output.get().is_empty() {
            set_dirty.set(true);
        }
        SpecialNonReactiveZone::exit(p);
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

        output.set(values);
        set_dirty.set(false);
    };

    // save code to local storage
    create_effect(move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            let code = input.get();
            if storage.set_item(STORAGE_KEY, &code).is_err() {
                logging::error!("error while trying to set item in localStorage");
            }
        }
    });

    view! {
        <div id="ide">
        <div id="header">
              <div class="text-align: left;">
                // <a href="file-chooser.html">"Scamper"</a>
                // " "
                <span id="version">{format!("({})", VERSION)}</span>
                " ⋅ "
                // <span id="current-file"></span>
                // " ⋅ "
                <button id="run" class="fa-solid fa-play" on:click=run_click></button>
                // " "
                // <button id="step" class="fa-solid fa-route" disabled></button>
                // " "
                // <button id="run-window" class="fa-solid fa-window-maximize" disabled></button>
                // " ⋅ "
                // <a href="docs/index.html">"Docs"</a>
                // " ⋅ "
                // <a href="reference.html">"Reference"</a>
              </div>
              <div class="text-align: right; font-size: 0.75em; color: #333;">
                <a href="https://github.com/cbratland/scamper-rs" target="_blank"><i class="fa-brands fa-github"></i></a> " ⋅ "
                <em><a href="https://github.com/cbratland/scamper-rs/issues" target="_blank">"Report an issue"</a></em>
              </div>
            </div>
            <div id="content">
                <CodeMirror
                    input=starting_input
                    on_change
                    node_ref=editor
                />
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
                            <em>(Warning: results out of sync with updated code)</em>
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

        // <div id="loading" style:display={move || if loading.get() { "block" } else { "none" }}>
        //     <div id="loading-content">
        //         "Loading Scamper.."
        //         <button
        //             on:click=move |_| {
        //                 set_loading.set(false);
        //             }
        //         >
        //             "close"
        //         </button>
        //     </div>
        // </div>
    }
}
