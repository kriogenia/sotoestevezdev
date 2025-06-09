use std::str::FromStr;

use strum::{EnumIter, EnumString, IntoEnumIterator};

use crate::github::get_repo;

const HEADER: &str = include_str!("../../static/projects.html");

pub(super) async fn run(arg: Option<&str>) -> Vec<String> {
    if matches!(arg, Some("ls") | None) {
        return HEADER
            .lines()
            .map(|s| s.to_owned())
            .chain(Project::iter().flat_map(format))
            .collect();
    }
    match Project::from_str(arg.unwrap()) {
        Ok(p) => p.print().await,
        Err(_) => vec!["Unknown project".to_string()],
    }
}

#[derive(strum::Display, EnumString, EnumIter)]
#[strum(serialize_all = "snake_case")]
enum Project {
    Rede,
    Portfolio,
    Archrio,
    Oito,
    Nove,
    Shotdown,
    #[strum(serialize = "tpos")]
    ThePartingOfSarah,
}

impl Project {
    fn name(&self) -> String {
        match self {
            Project::Rede => "Rede".to_string(),
            Project::Portfolio => "sotoestevez.dev".to_string(),
            Project::Archrio => "Archrio".to_string(),
            Project::Shotdown => "Shotdown".to_string(),
            Project::ThePartingOfSarah => "The Parting of Sarah".to_string(),
            _ => self.to_string(),
        }
    }

    fn repo(&self) -> String {
        match self {
            Project::Portfolio => "sotoestevezdev".to_string(),
            Project::ThePartingOfSarah => "the-parting-of-sarah".to_string(),
            _ => self.to_string(),
        }
    }

    fn description(&self) -> &str {
        use Project::*;
        match self {
            Rede => {
                "Open-source HTTP command line client to run suites of requests stored in Git-friendly files using a custom DSL"
            }
            Portfolio => "This page and all its secrets",
            Archrio => "Custom ArchLinux ISO and installer",
            Oito => {
                "Chip-8 emulator made in Rust compatible with web via WebAssembly compilation; and desktop with an SDL2 front-end"
            }
            Nove => "Unfinished NES emulator targetting the same capabilities of Oito",
            Shotdown => {
                "Simple two-player competitive shooting game based on Duck Game. Developed in custom engine running physics with Chipmunk2D"
            }
            ThePartingOfSarah => {
                "Basic top-view procedural rogue-like with rooms, bosses and items"
            }
        }
    }

    fn tags(&self) -> Vec<&str> {
        use Project::*;
        match self {
            Rede => vec!["rust", "network", "http", "command-line", "tokyo"],
            Portfolio => vec![
                "rust",
                "web-assembly",
                "javascript",
                "html",
                "css",
                "leptos",
            ],
            Archrio => vec!["linux", "arch", "shell", "sys-admin"],
            Oito => vec!["rust", "web-assembly", "sdl2", "emulation"],
            Nove => vec!["rust", "emulation"],
            Shotdown => vec!["c++", "videogame", "multiplayer"],
            ThePartingOfSarah => vec!["c++", "videogame", "procedural"],
        }
    }

    // TODO: website

    async fn print(&self) -> Vec<String> {
        let repo = get_repo(self.repo());
        let name = self.name();
        let tags = self
            .tags()
            .iter()
            .map(|t| format!("#{t}"))
            .collect::<Vec<String>>()
            .join(" ");

        let mut print = vec![
            format!(r#"<span class="project-title">{name}</span>"#),
            self.description().to_string(),
            format!(r#"<span class="tags">{tags}</span>"#),
        ];

        let repo = repo.await;
        macro_rules! section {
            ($title:literal: $val:tt) => {
                print.push(format!("  ❯ <em>{}</em>: {}", $title, $val));
            };
        }

        section!("Link": (format!(r#"<a href="{}">{}</a>"#, repo.url, repo.url)));

        let mut langs = repo.languages();
        if let Some((lang, count)) = langs.next() {
            section!("Languages": (format!("<strong>{lang}</strong> ({count})")));
        }
        for (lang, count) in langs {
            print.push(format!("               {lang} ({count})"));
        }

        if repo.forks > 0 {
            section!("Forks": (repo.forks));
        }

        if repo.watchers > 0 {
            section!("Watchers": (repo.watchers));
        }

        section!("Size": (format!("{} KB", repo.size)));
        section!("Created": (repo.created_at.split('T').next().unwrap_or_default()));

        print
    }
}

fn format(project: Project) -> Vec<String> {
    let name = project.name();
    let tags: String = project.tags().iter().map(|t| format!("  #{t}")).collect();
    vec![
        format!("<strong>{name}</strong> [<em>{project}</em>]"),
        format!("  {}", project.description()),
        format!(r#"<span class="tags">{tags}</span>"#),
    ]
}
