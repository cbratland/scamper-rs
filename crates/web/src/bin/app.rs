use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use web::bindings;
use web::components::{Docs, FileList, Ide};
use web::URL_PREFIX;

fn main() {
    console_error_panic_hook::set_once();

    provide_meta_context();

    bindings::init_player();

    mount_to_body(move || {
        view! {
            <Router base=URL_PREFIX trailing_slash=TrailingSlash::Redirect>
                <Routes>
                    <Route path=format!("{URL_PREFIX}/") view=FileList/>
                    <Route path=format!("{URL_PREFIX}/file/:file") view=Ide/>
                    <Route path=format!("{URL_PREFIX}/docs") view=Docs/>
                    <Route path=format!("{URL_PREFIX}/docs/:module") view=Docs/>
                </Routes>
            </Router>
        }
    })
}
