use std::{cell::RefCell, rc::Rc};

use crate::bindings::{create_editor, EditorView};
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
pub fn CodeMirror(
    input: Signal<Option<String>>,
    // errors: Signal<Vec<ErrorMarker>>,
    #[prop(into)] on_change: Callback<Option<String>, ()>,
    #[prop(into)] node_ref: NodeRef<html::Div>,
) -> impl IntoView {
    let editor_instance: Rc<RefCell<Option<EditorView>>> = Rc::new(RefCell::new(None));
    // let editor_instance_clone = editor_instance.clone();

    // let is_updating = Rc::new(RefCell::new(false));
    // let is_updating_clone = is_updating.clone();

    node_ref.on_load(move |node| {
        if let Some(element) = node.dyn_ref::<HtmlElement>().cloned() {
            let on_change = on_change.clone();
            // let is_updating = is_updating.clone();

            let onupdate = Closure::wrap(Box::new(move |editor: EditorView| {
                // if *is_updating.borrow() {
                //     return;
                // }
                let content = editor.get_doc();
                on_change.call(Some(content));
            }) as Box<dyn Fn(EditorView)>);

            let initial_content = input.get().unwrap_or_default();
            let editor = create_editor(&initial_content, element, &onupdate);
            *editor_instance.borrow_mut() = Some(editor);

            onupdate.forget(); // prevent closure from getting dropped
        }
    });

    // input changes
    // create_effect(move |_| {
    //     if let Some(new_content) = input.get() {
    //         if let Some(editor) = &*editor_instance_clone.borrow() {
    //             *is_updating_clone.borrow_mut() = true;
    //             editor.set_doc(&new_content);
    //             *is_updating_clone.borrow_mut() = false;
    //         }
    //     }
    // });

    view! {
        <div
            id="editor"
            _ref=node_ref
        />
    }
}
