use super::{CodeBlock, CompositionView, DrawingView, LabElementView};
use leptos::*;
use scamper_rs::ast::{FromValue, Value};
use scamper_rs::modules::image::{Drawing, Hsv, Rgb};
use scamper_rs::modules::lab::LabElement;
use scamper_rs::modules::music::Composition;

#[derive(Debug, Clone)]
pub enum ValueOrError {
    Value(Value),
    Error(String),
}

fn custom_view(value: &Value) -> Option<View> {
    match value {
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
            if let Some(rgb) = Rgb::from_value(&value) {
                return Some(render_rgb(rgb, rgb.to_string()));
            } else if let Some(hsv) = Hsv::from_value(&value) {
                return Some(render_rgb(hsv.to_rgb(), hsv.to_string()));
            }
        }
        Value::Foreign(item) => {
            if let Some(drawing) = item.downcast_ref::<Drawing>() {
                return Some(
                    view! {
                        <DrawingView drawing=drawing.clone() />
                    }
                    .into_view(),
                );
            } else if let Some(element) = item.downcast_ref::<LabElement>() {
                return Some(
                    view! {
                        <LabElementView element=element.clone() />
                    }
                    .into_view(),
                );
            } else if let Some(composition) = item.downcast_ref::<Composition>() {
                return Some(
                    view! {
                        <CompositionView composition=composition.clone() />
                    }
                    .into_view(),
                );
            }
        }
        _ => {}
    }
    None
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
        ValueOrError::Value(value) => {
            if let Some(view) = custom_view(&value) {
                view
            } else {
                match value {
                    Value::List(values) => view! {
                        <code class="hljs">
                            <CodeBlock>
                                "(list"
                            </CodeBlock>
                            <For
                                each=move || values.clone()
                                key=|value| format!("{:?}", value)
                                children=move |value| {
                                    let mut nodes = vec![view! {
                                        <CodeBlock>" "</CodeBlock>
                                    }.into_view()];

                                    nodes.push(if let Some(view) = custom_view(&value) {
                                        view
                                    } else {
                                        view! {
                                            {value.to_string()}
                                        }.into_view()
                                    });

                                    nodes
                                }
                            />
                            <CodeBlock>
                                ")"
                            </CodeBlock>
                        </code>
                    }
                    .into_view(),
                    Value::Pair(px, py) => view! {
                        <code class="hljs">
                            <CodeBlock>
                                "(pair "
                            </CodeBlock>
                            {if let Some(view) = custom_view(&px) {
                                view
                            } else {
                                view! {
                                    {px.to_string()}
                                }.into_view()
                            }}
                            <CodeBlock>
                                " "
                            </CodeBlock>
                            {if let Some(view) = custom_view(&py) {
                                view
                            } else {
                                view! {
                                    {py.to_string()}
                                }.into_view()
                            }}
                            <CodeBlock>
                                ")"
                            </CodeBlock>
                        </code>
                    }
                    .into_view(),
                    _ => view! {
                        <CodeBlock>
                            {value.to_string()}
                        </CodeBlock>
                    }
                    .into_view(),
                }
            }
        }
    }
}
