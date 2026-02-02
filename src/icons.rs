use strum::EnumString;

#[derive(strum::Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Icon {
    #[strum(
        serialize = "shell",
        serialize = "just",
        to_string = r#"<i class="devicon-bash-plain"></i>"#
    )]
    Bash,
    #[strum(serialize = "c", to_string = r#"<i class="devicon-c-plain"></i>"#)]
    C,
    #[strum(
        serialize = "cmake",
        serialize = "makefile",
        to_string = r#"<i class="devicon-cmake-plain"></i>"#
    )]
    CMake,
    #[strum(
        serialize = "c++",
        to_string = r#"<i class="devicon-cplusplus-plain"></i>"#
    )]
    CPlusPlus,
    #[strum(serialize = "css", to_string = r#"<i class="devicon-css3-plain"></i>"#)]
    Css,
    #[strum(
        serialize = "github",
        to_string = r#"<i class="devicon-github-original"></i>"#
    )]
    GitHub,
    #[strum(
        serialize = "gleam",
        to_string = r#"<i class="devicon-gleam-plain"></i>"#
    )]
    Gleam,
    #[strum(
        serialize = "html",
        to_string = r#"<i class="devicon-html5-plain"></i>"#
    )]
    Html,
    #[strum(
        serialize = "javascript",
        to_string = r#"<i class="devicon-javascript-plain"></i>"#
    )]
    Javascript,
    #[strum(
        serialize = "python",
        to_string = r#"<i class="devicon-python-plain"></i>"#
    )]
    Python,
    #[strum(
        serialize = "rust",
        to_string = r#"<i class="devicon-rust-original"></i>"#
    )]
    Rust,
    #[strum(
        serialize = "typescript",
        to_string = r#"<i class="devicon-typescript-plain"></i>"#
    )]
    Typescript,
    #[strum(to_string = r#"<i class="devicon-wasm-original"></i>"#)]
    WebAssembly,
    #[strum(serialize = "zig", to_string = r#"<i class="devicon-zig-plain"></i>"#)]
    Zig,
}
