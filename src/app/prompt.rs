use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use leptos::{IntoView, component, view};
use log::debug;

use crate::shell::Shell;

#[component]
pub fn Prompt(buffer: RwSignal<Vec<String>>) -> impl IntoView {
    let mut shell = Shell::default();

    let (input, set_input) = signal(String::new());

    let on_key = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            let value = input.get();
            for line in shell.interpret(value) {
                debug!("{:?}", line);
                buffer.update(|buf| buf.push(line));
                debug!("{:?}", buffer.read().to_vec());
            }
            set_input.set(String::new());
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
