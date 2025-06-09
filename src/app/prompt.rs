use leptos::ev::KeyboardEvent;
use leptos::html::Input;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{IntoView, component, view};
use wasm_bindgen::UnwrapThrowExt;

use crate::app::history::History;
use crate::index::{print_output, print_prompt};
use crate::shell;

pub const PROMPT: &str = include_str!("../../static/prompt.html");

#[component]
pub fn Prompt() -> impl IntoView {
    let history = RwSignal::new(History::default());

    Effect::new(move || print_output(shell::greet()));

    let input: NodeRef<Input> = NodeRef::new();
    let on_key = move |ev: KeyboardEvent| {
        let key = ev.key();
        let input = input.get().unwrap_throw();
        match key.as_ref() {
            "Enter" => {
                let value = input.value();
                history.update(|h| h.push(&value));
                print_prompt(&format!("{PROMPT}{value}"));
                spawn_local(async move {
                    print_output(shell::interpret(value).await);
                });
                input.set_value("");
            }
            "ArrowUp" => {
                ev.prevent_default();
                if let Some(prev) = history.write().prev() {
                    input.set_value(prev);
                }
            }
            "ArrowDown" => {
                ev.prevent_default();
                if let Some(next) = history.write().next() {
                    input.set_value(next);
                }
            }
            "Tab" => {
                ev.prevent_default();
                let opts = shell::autocomplete_options(&input.value());
                if opts.len() == 1 {
                    input.set_value(&opts[0]);
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
