use leptos::prelude::*;
use leptos::{IntoView, component, view};

#[component]
pub fn TTY(buffer: RwSignal<Vec<String>>) -> impl IntoView {
    let mut next_id = 0;
    let (lines, set_lines) = signal::<Vec<(i32, String)>>(vec![]);

    Effect::new(move |_| {
        for line in buffer.get().into_iter() {
            set_lines.update(|lines| lines.push((next_id, line)));
            next_id += 1;
        }
        buffer.update_untracked(|buf| buf.clear());
    });

    view! {
        <div id="buffer" class="buffer">
            <For
                each=move || lines.get()
                key=|counter| counter.0
                children=move |(_, line)| {
                    view! { <p inner_html=line/> }
                }
            />
        </div>
    }
}
