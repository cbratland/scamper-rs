use leptos::*;

mod components;
use components::Ide;

const VERSION: &str = "0.1.0";

fn main() {
    mount_to_body(move || {
        view! {
            <Ide />
        }
    })
}
