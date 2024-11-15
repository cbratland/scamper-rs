pub mod bindings;
pub mod components;
pub mod fs_worker;

pub const VERSION: &str = "0.1.0";
pub const URL_PREFIX: &str = if let Some(prefix) = option_env!("URL_PREFIX") {
    prefix
} else {
    ""
};
