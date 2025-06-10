use wasm_bindgen::prelude::*;

const HEADER: &str = include_str!("../../static/themes.html");

pub(super) fn run(arg: Option<&str>) -> Vec<String> {
    match arg.map(Theme::from) {
        Some(Theme::Unknown) => vec!["Unknown theme".to_string()],
        Some(Theme::List) | None => HEADER
            .lines()
            .map(std::borrow::ToOwned::to_owned)
            .chain(Theme::list().iter().map(format))
            .collect(),
        Some(theme) => {
            let class = theme.class();
            set_theme(class);
            let extra = if theme.is_dark() {
                ""
            } else {
                " (a light theme... <strong>DISGUSTING</strong>)"
            };
            vec![format!("Swapping theme to {}{}", class, extra)]
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setTheme)]
    fn set_theme(theme: &str);
}

#[derive(PartialEq)]
enum Theme {
    Dracula,
    Gruvbox,
    Hearthian,
    Latte,
    Mocha,
    Monochrome,
    OldSchool,
    Tokyonight,
    List,
    Remove,
    Unknown,
}

impl From<&str> for Theme {
    fn from(value: &str) -> Self {
        let name = value.trim();
        match name {
            "catppuccin-mocha" | "mocha" => Self::Mocha,
            "dracula" => Self::Dracula,
            "gruvbox" => Self::Gruvbox,
            "hearthian" => Self::Hearthian,
            "monochrome" => Self::Monochrome,
            "latte" => Self::Latte,
            "ls" => Self::List,
            "oldschool" => Self::OldSchool,
            "rm" => Self::Remove,
            "tokyonight" => Self::Tokyonight,
            _ => Self::Unknown,
        }
    }
}

impl Theme {
    fn class(&self) -> &str {
        match self {
            Self::Dracula => "dracula",
            Self::Gruvbox => "gruvbox",
            Self::Hearthian => "hearthian",
            Self::Latte => "catppuccin-latte",
            Self::Mocha => "catppuccin-mocha",
            Self::Monochrome => "monochrome",
            Self::OldSchool => "oldschool",
            Self::Remove => "wait what? Ooooh, well played",
            Self::Tokyonight => "tokyonight",
            Self::List | Self::Unknown => unreachable!("only requested in value theme names"),
        }
    }

    fn name(&self) -> &str {
        match self {
            Self::Latte => "latte | catppuccin-latte",
            Self::Mocha => "mocha | catppuccin-mocha",
            _ => self.class(),
        }
    }

    fn list() -> Vec<Theme> {
        vec![
            Self::Dracula,
            Self::Gruvbox,
            Self::Hearthian,
            Self::Latte,
            Self::Mocha,
            Self::Monochrome,
            Self::OldSchool,
            Self::Tokyonight,
        ]
    }

    fn is_dark(&self) -> bool {
        !matches!(self, Self::Latte)
    }
}

fn format(theme: &Theme) -> String {
    format!("  <em>{}</em>", theme.name())
}
