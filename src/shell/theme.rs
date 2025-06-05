use wasm_bindgen::prelude::*;

pub(super) fn run(arg: Option<&str>) -> Vec<String> {
    match arg.map(Themes::from) {
        Some(Themes::Unknown) => vec!["Unknown theme".to_string()],
        Some(theme) => {
            let class = theme.class();
            set_theme(class);
            vec![format!("Swapping theme to {}", class)]
        }
        None => vec!["list of themes".to_string()],
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTheme)]
    fn set_theme(theme: &str);
}

#[derive(PartialEq)]
enum Themes {
    Hearthian,
    Mocha,
    OldSchool,
    // TODO: add dracula, tokyonight, latte?, nordic?
    Unknown,
}

impl From<&str> for Themes {
    fn from(value: &str) -> Self {
        let name = value.trim();
        match name {
            "hearthian" => Self::Hearthian,
            "mocha" => Self::Mocha,
            "oldschool" => Self::OldSchool,
            _ => Self::Unknown,
        }
    }
}

impl Themes {
    fn class(&self) -> &str {
        match self {
            Self::Hearthian => "hearthian",
            Self::Mocha => "catppuccin-mocha",
            Self::OldSchool => "oldschool",
            Self::Unknown => "",
        }
    }
}
