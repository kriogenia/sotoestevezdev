use strum::{EnumIter, EnumString};

#[derive(EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
pub(super) enum Command {
    About,
    Cat,
    Cd,
    Clear,
    Cp,
    Contact,
    Echo,
    #[strum(serialize = "vi", serialize = "vim", serialize = "nvim")]
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
    #[strum(serialize = "ed", serialize = "emacs", serialize = "nano")]
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

