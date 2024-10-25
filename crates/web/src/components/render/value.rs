use super::{CodeBlock, DrawingView};
use leptos::*;
use scamper_rs::ast::Value;
use scamper_rs::modules::image::{Drawing, Rgb};

#[component]
pub fn RenderedValue(value: Value) -> impl IntoView {
    match value {
        Value::Foreign(item) => {
            if let Some(drawing) = item.downcast_ref::<Drawing>() {
                return view! {
                    <DrawingView drawing=drawing.clone() />
                }
                .into_view();
            }
            if let Some(rgb) = item.downcast_ref::<Rgb>() {
                let color = rgb.to_string();
                let text_color = rgb.pseudo_complement().to_string();
                return view! {
                	<div
                 		style=format!(
                   			"color: {}; background-color: {}; width: fit-content; border: 1px solid black; padding: 0.25em;",
                      		text_color, color
                   		)
                 	>
						{color}
					</div>
                }
                .into_view();
            } else {
                return view! {
                    <CodeBlock>
                        "<foreign>"
                    </CodeBlock>
                }
                .into_view();
            }
        }
        _ => {}
    };

    view! {
        <CodeBlock>
            {value.to_string()}
        </CodeBlock>
    }
}
