use leptos::ev::KeyboardEvent;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::{IntoView, component, view};
use log::debug;
use wasm_bindgen::UnwrapThrowExt;

use crate::index::{print_output, print_prompt};
use crate::shell::Shell;

pub const PROMPT: &str = include_str!("../../static/prompt.html");

#[component]
pub fn Prompt() -> impl IntoView {
    let mut shell = Shell::default();

    let greet = shell.greet();
    Effect::new(move || print_output(greet.clone()));

    let input: NodeRef<Input> = NodeRef::new();
    let on_key = move |ev: KeyboardEvent| {
        let key = ev.key();
        let input = input.get().unwrap_throw();
        match key.as_ref() {
            "Enter" => {
                let value = input.value();
                print_prompt(&format!("{PROMPT}{value}"));
                print_output(shell.interpret(value));
                input.set_value("");
            }
            "ArrowUp" => {
                if let Some(prev) = shell.prev() {
                    input.set_value(prev);
                    let caret = prev.len() as u32;
                    input.set_selection_range(caret, caret).unwrap_throw();
                }
            }
            _ => {}
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
                node_ref=input
                on:keydown=on_key
            />
        </p>
    }
}
