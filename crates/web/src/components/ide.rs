use super::{CodeMirror, RenderedValue};
use crate::VERSION;
use leptos::*;
use scamper_rs::Engine;

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
        .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten());
    let input = RwSignal::new(starting_input);
    let output = RwSignal::new(Vec::new());

    // update stored input code when input changes
    let on_change = Callback::new(move |txt: Option<String>| {
        input.set(txt.clone());
    });

    // let errors = RwSignal::new(Vec::new());

    // handle run button click
    let run_click = move |_| {
        if let Some(code) = input.get() {
            let engine = Engine::new();

            let Ok(scm_output) = engine.run(&code) else {
                logging::error!("error while trying to execute code");
                return;
            };

            output.set(scm_output);
            set_dirty.set(false);
        }
    };

    // save code to local storage
    create_effect(move |_| {
        if let Ok(Some(storage)) = window().local_storage() {
            let code = input.get().unwrap_or_default();
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
              // <div class="text-align: right; font-size: 0.75em; color: #333;">
              //   <a href="https://github.com/slag-plt/scamper"><i class="fa-brands fa-github"></i></a> " ⋅ "
              //   <em><a href="https://github.com/slag-plt/scamper/issues">"Report an issue"</a></em>
              // </div>
            </div>
            <div id="content">
                <div id="editor" style="width: calc(65% - 5px);">
                    <CodeMirror
                      input=input.into()
                      on_change
                      // errors = errors.into()
                    />
                </div>
                <div class="gutter gutter-horizontal" style="width: 10px;"></div>
                <div id="results" style="width: calc(35% - 5px);">
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
