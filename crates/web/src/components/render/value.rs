use super::{CodeBlock, DrawingView};
use leptos::*;
use scamper_rs::ast::{FromValue, Value};
use scamper_rs::modules::image::{Drawing, Hsv, Rgb};

#[derive(Debug, Clone)]
pub enum ValueOrError {
    Value(Value),
    Error(String),
}

#[component]
pub fn RenderedValue(value: ValueOrError) -> impl IntoView {
    match value {
        ValueOrError::Error(error) => {
            return view! {
                <CodeBlock>
                    {error}
                </CodeBlock>
            }
            .into_view()
        }
        ValueOrError::Value(ref value) => match value {
            Value::Struct(_) => {
                fn render_rgb(rgb: Rgb, text: String) -> View {
                    let bg_color = rgb.to_string();
                    let text_color = rgb.pseudo_complement().to_string();
                    return view! {
	                	<div
	                 		style=format!(
	                   			"color: {}; background-color: {}; width: fit-content; border: 1px solid black; padding: 0.25em;",
	                      		text_color, bg_color
	                   		)
	                 	>
							{text}
						</div>
	                }
	                .into_view();
                }
                if let Some(rgb) = Rgb::from_value(value) {
                    return render_rgb(rgb, rgb.to_string());
                } else if let Some(hsv) = Hsv::from_value(value) {
                    return render_rgb(hsv.to_rgb(), hsv.to_string());
                }
            }
            Value::Foreign(item) => {
                if let Some(drawing) = item.downcast_ref::<Drawing>() {
                    return view! {
                        <DrawingView drawing=drawing.clone() />
                    }
                    .into_view();
                }
            }
            _ => {}
        },
    }

    let ValueOrError::Value(value) = value else {
        unreachable!()
    };
    view! {
        <CodeBlock>
            {value.to_string()}
        </CodeBlock>
    }
    .into_view()
}
