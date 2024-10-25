use leptos::html::Code;
use leptos::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast};
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/public/js/highlight.js")]
extern "C" {
    #[wasm_bindgen(js_name = highlightElement)]
    fn highlight_element(element: &HtmlElement);
}

#[component]
pub fn CodeBlock(
    children: Children,
    #[prop(default = "scheme")] language: &'static str,
) -> impl IntoView {
    let node_ref = create_node_ref::<Code>();

    create_effect(move |_| {
        if let Some(element) = node_ref.get() {
            if let Some(el) = element.dyn_ref::<HtmlElement>() {
                highlight_element(el)
            }
        }
    });

    view! {
        <code node_ref=node_ref class=format!("hljs language-{language}")>
            {children()}
        </code>
    }
}
