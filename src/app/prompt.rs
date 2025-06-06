use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use leptos::{IntoView, component, view};

use crate::index::{print_output, print_prompt};
use crate::shell::Shell;

pub const PROMPT: &str = include_str!("../../static/prompt.html");

#[component]
pub fn Prompt() -> impl IntoView {
    let mut shell = Shell::default();

    let (input, set_input) = signal(String::new());
    let on_key = move |ev: KeyboardEvent| {
        if ev.key() == "Enter" {
            let value = input.get();
            print_prompt(&format!("{PROMPT}{value}"));
            print_output(shell.interpret(value));
            set_input.set(String::new());
        }
    };

    view! {
        <p class="prompt">
            <span inner_html=PROMPT/>
            <input
                id="input-prompt"
                type="text"
                enterkeyhint="Enter"
                spellcheck="false"
                autocapitalize="none"
                autocomplete="off"
                prop:value=input
                on:input=move |ev| set_input.set(event_target_value(&ev))
                on:keydown=on_key
            />
        </p>
    }
}
