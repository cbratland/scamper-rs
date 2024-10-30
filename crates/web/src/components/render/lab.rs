use leptos::*;
use scamper_rs::modules::lab::LabElement;

#[component]
pub fn LabElementView(element: LabElement) -> impl IntoView {
    match element {
        LabElement::Title(text) => view! {
            <h1>{text}</h1>
        }
        .into_view(),
        LabElement::Part(text) => view! {
            <h2>{text}</h2>
        }
        .into_view(),
        LabElement::Problem(text) => view! {
            <h3>{text}</h3>
        }
        .into_view(),
        LabElement::Description(text) => view! {
            <p><em>{text}</em></p>
        }
        .into_view(),
    }
}
