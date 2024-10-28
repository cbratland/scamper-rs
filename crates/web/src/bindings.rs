use leptos::*;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/build/index.js")]
extern "C" {
    #[must_use]
    #[derive(Debug, Clone, PartialEq)]
    pub type EditorView;

    #[wasm_bindgen(js_name = createEditor)]
    pub fn create_editor(
        doc: &str,
        parent: HtmlElement,
        onupdate: &Closure<dyn Fn(EditorView)>,
    ) -> EditorView;

    #[wasm_bindgen(method)]
    pub fn get_doc(this: &EditorView) -> String;

    #[wasm_bindgen(method)]
    pub fn set_doc(this: &EditorView, content: &str);

    #[wasm_bindgen(js_name = createSplit)]
    pub fn create_split(elements: Vec<HtmlElement>, sizes: Vec<f64>);
}
