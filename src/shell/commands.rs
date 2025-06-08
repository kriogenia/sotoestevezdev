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
    Greeting,
    #[strum(serialize = "grep", serialize = "rg")]
    Grep,
    Help,
    History,
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
        use Command::*;
        match self {
            About => Some("info about me"),
            Clear => Some("clears the screen"),
            Contact => Some("list my profiles"),
            Exit => Some("back to the boring static page"),
            Greeting => Some("display again the console greeting message"),
            Help => Some("shows this exact help"),
            History => Some("display all commands you used"),
            Theme => Some("changes the color theme"),
            _ => None,
        }
    }
}
