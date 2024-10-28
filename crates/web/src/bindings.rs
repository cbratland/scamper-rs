use leptos::*;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/build/index.js")]
extern "C" {
    #[must_use]
    #[derive(Debug, Clone, PartialEq)]
    pub type EditorView;

    #[must_use]
    #[derive(Debug, Clone, PartialEq)]
    pub type Diagnostic;

    #[wasm_bindgen(js_name = createEditor)]
    pub fn create_editor(
        doc: &str,
        parent: HtmlElement,
        onupdate: &Closure<dyn Fn(EditorView)>,
        onlinting: &Closure<dyn Fn(EditorView) -> Vec<Diagnostic>>,
    ) -> EditorView;

    #[wasm_bindgen(method)]
    pub fn get_doc(this: &EditorView) -> String;

    #[wasm_bindgen(method)]
    pub fn set_doc(this: &EditorView, content: &str);

    #[wasm_bindgen(js_name = createDiagnostic)]
    pub fn create_diagnostic(from: u32, to: u32, severity: String, message: String) -> Diagnostic;

    #[wasm_bindgen(js_name = createSplit)]
    pub fn create_split(elements: Vec<HtmlElement>, sizes: Vec<f64>);
}
