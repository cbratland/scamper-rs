use codemirror::{DocApi, Editor, EditorOptions, GutterId};
use leptos::*;
use leptos::{
    html::Textarea,
    // web_sys::{Document, Element},
};
use std::rc::Rc;

const GUTTER_ERROR: GutterId = GutterId::new("gutter-error");

// #[derive(Debug, Clone, PartialEq)]
// pub struct ErrorMarker {
//     pub line: Line,
// }

#[component]
pub fn CodeMirror(
    input: Signal<Option<String>>,
    // errors: Signal<Vec<ErrorMarker>>,
    #[prop(into)] on_change: Callback<Option<String>, ()>,
) -> impl IntoView {
    let textarea_ref = NodeRef::<Textarea>::new();

    textarea_ref.on_load(move |el| {
        let _ = el.on_mount(move |el| {
            let options = EditorOptions::default()
                .line_numbers(true)
                .gutters(&[GUTTER_ERROR]);
            let editor = Editor::from_text_area(&el, &options);

            editor.set_value(&match input.get() {
                Some(v) => v,
                None => String::default(),
            });

            editor.on_change(move |editor, _| {
                let value = editor.value();
                on_change.call(value);
            });

            let editor = Rc::new(editor);

            // create_effect({
            //     let editor = Rc::clone(&editor);
            //     move |_| {
            //         editor.set_value(&match input.get() {
            //             Some(v) => v,
            //             None => String::default(),
            //         });
            //     }
            // });

            create_effect(move |_| {
                input.with(|x| {
                    let txt = match x {
                        Some(v) => v,
                        None => "",
                    };
                    if editor.value() != *x {
                        editor.set_value(txt);
                    }
                });
            });
            // create_effect({
            //     // let editor = Rc::clone(&editor);
            //     move |_| {
            //         input.try_with(|x| {
            //             let txt = match x {
            //                 Some(v) => v,
            //                 None => "",
            //             };
            //             if editor.value() != *x {
            //                 editor.set_value(txt);
            //             }
            //         });
            //     }
            // });

            // Effect::new(move |_| {
            //     errors.with(|errors| {
            //         editor.clear_gutter(GUTTER_ERROR);
            //         let doc = document();
            //         for ErrorMarker { line } in errors {
            //             let marker = create_error_marker(&doc);
            //             editor.set_gutter_marker(*line, GUTTER_ERROR, &marker);
            //         }
            //     });
            // });
        });
    });

    view! { <textarea _ref=textarea_ref /> }
}

// const ERROR_MARKER_CLASS: &str = "CodeMirror-lint-marker-error CodeMirror-lint-marker";

// fn create_error_marker(document: &Document) -> Element {
//     let marker = document.create_element("div").unwrap();
//     marker.set_attribute("class", ERROR_MARKER_CLASS).unwrap();
//     marker
// }
