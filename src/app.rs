mod prompt;
mod tty;

use leptos::prelude::*;
use leptos::{IntoView, component, view};
use prompt::Prompt;
use tty::TTY;

#[component]
pub fn App() -> impl IntoView {
    let (new_line, set_new_lines) = signal::<Option<String>>(None);

    view! {
        <div id="app" class="app hidden">
            <TTY read_line=new_line/>
            <Prompt send_line=set_new_lines/>
        </div>
    }
}
