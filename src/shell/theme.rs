use wasm_bindgen::prelude::*;

const HEADER: &str = include_str!("../../static/themes.html");

pub(super) fn run(arg: Option<&str>) -> Vec<String> {
    match arg.map(Themes::from) {
        Some(Themes::Unknown) => vec!["Unknown theme".to_string()],
        Some(Themes::List) | None => HEADER
            .lines()
            .map(|s| s.to_owned())
            .chain(Themes::list().iter().map(format))
            .collect(),
        Some(theme) => {
            let class = theme.class();
            set_theme(class);
            vec![format!("Swapping theme to {}", class)]
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTheme)]
    fn set_theme(theme: &str);
}

#[derive(PartialEq)]
enum Themes {
    Dracula,
    Hearthian,
    Mocha,
    OldSchool,
    Tokyonight,
    // TODO: latte?, nordic?, gruvbox
    List,
    Unknown,
}

impl From<&str> for Themes {
    fn from(value: &str) -> Self {
        let name = value.trim();
        match name {
            "catppuccin-mocha" | "mocha" => Self::Mocha,
            "dracula" => Self::Dracula,
            "hearthian" => Self::Hearthian,
            "ls" => Self::List,
            "oldschool" => Self::OldSchool,
            "tokyonight" => Self::Tokyonight,
            _ => Self::Unknown,
        }
    }
}

impl Themes {
    fn class(&self) -> &str {
        match self {
            Self::Dracula => "dracula",
            Self::Hearthian => "hearthian",
            Self::Mocha => "catppuccin-mocha",
            Self::OldSchool => "oldschool",
            Self::Tokyonight => "tokyonight",
            Self::List | Self::Unknown => unreachable!("only requested in value theme names"),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Mocha => "mocha | catppuccin-mocha",
            _ => self.class(),
        }
    }

    fn list() -> Vec<Themes> {
        vec![
            Self::Dracula,
            Self::Hearthian,
            Self::Mocha,
            Self::OldSchool,
            Self::Tokyonight,
        ]
    }
}

fn format(theme: &Themes) -> String {
    format!("  <em>{}</em>", theme.name())
}
