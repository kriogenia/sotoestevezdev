use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn Prompt(send_line: WriteSignal<Option<String>>) -> impl IntoView {
    let (input, set_input) = signal(String::new());

    let on_key = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            let value = input.get();
            if !value.trim().is_empty() {
                send_line.write().replace(value);
                set_input.set(String::new());
            }
        }
    };

    view! {
        <input
            class="prompt"
            type="text"
            prop:value=input
            on:input=move |ev| set_input.set(event_target_value(&ev))
            on:keydown=on_key
        />
    }
}
