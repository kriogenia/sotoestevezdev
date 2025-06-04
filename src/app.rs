mod prompt;
mod tty;

use leptos::prelude::*;
use leptos::{IntoView, component, view};
use prompt::Prompt;
use tty::TTY;

#[component]
pub fn App() -> impl IntoView {
    let buffer: RwSignal<Vec<String>> = RwSignal::new(Vec::new());

    view! {
        <div id="app" class="app hidden">
            <TTY buffer=buffer />
            <Prompt buffer=buffer />
        </div>
    }
}
