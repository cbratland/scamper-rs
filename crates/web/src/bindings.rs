use leptos::*;
use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen};
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/build/index.js")]
extern "C" {
    // CodeMirror
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

    // split.js
    #[wasm_bindgen(js_name = createSplit)]
    pub fn create_split(elements: Vec<HtmlElement>, sizes: Vec<f64>);

    // webaudiofont
    #[must_use]
    #[derive(Debug, Clone, PartialEq)]
    pub type Player;

    #[wasm_bindgen(js_name = initPlayer2)]
    pub fn init_player();

    #[wasm_bindgen(js_name = getPlayer2)]
    pub fn get_player() -> Player;

    #[wasm_bindgen(method, js_name = playNote)]
    pub fn play_note(
        this: &Player,
        time: f64,
        duration: f64,
        note: f64,
        instrument: u8,
        velocity: f64,
    );

    #[wasm_bindgen(method)]
    pub fn stop(this: &Player);

    #[wasm_bindgen(method, js_name = getTime)]
    pub fn get_time(this: &Player) -> f64;
}
