mod prompt;

use leptos::prelude::*;
use leptos::{IntoView, component, view};
use prompt::Prompt;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="app" class="app hidden">
            <div id="buffer" class="buffer"></div>
            <Prompt />
        </div>
    }
}
