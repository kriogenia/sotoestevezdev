// #![warn(clippy::pedantic)]
#![allow(unreachable_patterns)]

mod app;
mod github;
mod icons;
mod index;
mod shell;

use app::App;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
