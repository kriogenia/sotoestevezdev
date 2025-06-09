use strum::{EnumIter, EnumString};

#[allow(unreachable_patterns)]
#[derive(strum::Display, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
pub(super) enum Command {
    About,
    Cat,
    Cd,
    Clear,
    Cp,
    Contact,
    Echo,
    #[strum(
        serialize = "vi",
        serialize = "vim",
        serialize = "nvim",
        to_string = ""
    )]
    Editor,
    #[strum(disabled)]
    Empty,
    Exit,
    Github,
    Greeting,
    #[strum(serialize = "grep", serialize = "rg")]
    Grep,
    Help,
    Hostname,
    Ls,
    #[strum(
        serialize = "ed",
        serialize = "emacs",
        serialize = "nano",
        to_string = ""
    )]
    MissingEditor,
    #[strum(serialize = "mkdir")]
    MkDir,
    Mv,
    Project,
    Pwd,
    Rm,
    #[strum(serialize = "rmdir")]
    RmDir,
    #[strum(serialize = "su", serialize = "sudo")]
    Sudo,
    Theme,
    #[strum(disabled)]
    Unknown,
}

impl Command {
    pub fn help(&self) -> Option<&str> {
        use Command::{About, Clear, Contact, Exit, Github, Greeting, Help, Project, Theme};
        match self {
            About => Some("info about me"),
            Clear => Some("clears the screen"),
            Contact => Some("list my profiles"),
            Exit => Some("back to the boring static page"),
            Github => Some("show my github profile"),
            Greeting => Some("display again the console greeting message"),
            Help => Some("shows this exact help"),
            Project => Some("list my projects and their info"),
            Theme => Some("change the color theme"),
            _ => None,
        }
    }
}
