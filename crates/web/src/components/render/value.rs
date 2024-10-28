use super::{CodeBlock, DrawingView};
use leptos::*;
use scamper_rs::ast::Value;
use scamper_rs::modules::image::{Drawing, Rgb};

#[derive(Debug, Clone)]
pub enum ValueOrError {
    Value(Value),
    Error(String),
}

#[component]
pub fn RenderedValue(value: ValueOrError) -> impl IntoView {
    match value {
        ValueOrError::Error(error) => view! {
            <CodeBlock>
                {error}
            </CodeBlock>
        }
        .into_view(),
        ValueOrError::Value(value) => match value {
            Value::Foreign(item) => {
                if let Some(drawing) = item.downcast_ref::<Drawing>() {
                    view! {
                        <DrawingView drawing=drawing.clone() />
                    }
                    .into_view()
                } else if let Some(rgb) = item.downcast_ref::<Rgb>() {
                    let color = rgb.to_string();
                    let text_color = rgb.pseudo_complement().to_string();
                    view! {
	                	<div
	                 		style=format!(
	                   			"color: {}; background-color: {}; width: fit-content; border: 1px solid black; padding: 0.25em;",
	                      		text_color, color
	                   		)
	                 	>
							{color}
						</div>
	                }
	                .into_view()
                } else {
                    view! {
                        <CodeBlock>
                            "<foreign>"
                        </CodeBlock>
                    }
                    .into_view()
                }
            }
            _ => view! {
                <CodeBlock>
                    {value.to_string()}
                </CodeBlock>
            }
            .into_view(),
        },
    }
}
