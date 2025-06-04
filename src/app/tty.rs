use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn TTY(read_line: ReadSignal<Option<String>>) -> impl IntoView {
    let mut next_id = 0;
    let (lines, set_lines) = signal::<Vec<(i32, String)>>(vec![]);

    Effect::new(move |_| {
        if let Some(line) = read_line.get() {
            set_lines.update(|lines| lines.push((next_id, line)));
            next_id += 1;
        }
    });

    view! {
        <div id="buffer" class="buffer">
        <For
            each = move || lines.get()
            key = |counter| counter.0
            children = move |(_, line)| {
                    view! {<p>{line}</p>}
            }
        />
        </div>
    }
}
