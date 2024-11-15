use gloo_worker::Registrable;
use web::fs_worker::FsWorker;

fn main() {
    console_error_panic_hook::set_once();
    FsWorker::registrar().register();
}
