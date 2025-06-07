use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name = toggleTty)]
    pub fn closeTty();

    #[wasm_bindgen(js_name = printLine)]
    fn print_line(content: &str, delay: usize, class: Option<&str>);

    #[wasm_bindgen(js_name = clear)]
    pub fn clear();
}

pub fn print_prompt(content: &str) {
    print_line(content, 0, Some("echo"));
}

pub fn print_output(content: Vec<String>) {
    for (i, line) in content.into_iter().enumerate() {
        print_line(&line, i * 40, None);
    }
}
