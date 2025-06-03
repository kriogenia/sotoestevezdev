use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="app" class="hidden">
            <p>Hello World!</p>
        </div>
    }
}
