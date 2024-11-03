use leptos::*;

pub mod bindings;
mod components;
use components::Ide;

const VERSION: &str = "0.1.0";

fn main() {
    console_error_panic_hook::set_once();

    bindings::init_player();

    mount_to_body(move || {
        view! {
            <Ide />
        }
    })
}
