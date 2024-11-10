use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod bindings;
mod components;
use components::{Docs, FileList, Ide};

const VERSION: &str = "0.1.0";

fn main() {
    console_error_panic_hook::set_once();

    provide_meta_context();

    bindings::init_player();

    mount_to_body(move || {
        view! {
            <Router>
                <Routes>
                    <Route path="/" view=FileList/>
                    <Route path="/file/:file" view=Ide/>
                    <Route path="/docs" view=Docs/>
                    <Route path="/docs/:module" view=Docs/>
                </Routes>
            </Router>
        }
    })
}
