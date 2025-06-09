use std::collections::BTreeMap;

use futures::try_join;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Debug, Deserialize)]
pub(super) struct Repository {
    #[serde(rename = "html_url")]
    pub url: String,
    pub forks: i32,
    pub watchers: i32,
    pub size: i32,
    pub topics: Vec<String>,
    pub created_at: String,
    #[serde(default)]
    langs: BTreeMap<String, i32>,
}

impl Repository {
    pub fn languages(&self) -> impl Iterator<Item = (String, String)> {
        let total = self.langs.values().sum::<i32>() as f32;
        let mut sorted: Vec<_> = self.langs.iter().collect();
        sorted.sort_by_key(|&(_, count)| std::cmp::Reverse(count));
        sorted
            .into_iter()
            .map(move |(lang, code)| (lang, *code as f32 * 100.0 / total))
            .map(|(lang, percent)| (lang.clone(), format!("{percent:.1}%")))
    }
}

pub async fn get_repo(repo: impl AsRef<str>) -> Repository {
    let url = format!("https://api.github.com/repos/kriogenia/{}", repo.as_ref());
    let langs = format!("{url}/languages");

    let repo = Request::get(&url).send();
    let langs = Request::get(&langs).send();
    let (repo, langs) = try_join!(repo, langs).unwrap_throw();

    let mut repo: Repository = repo.json().await.unwrap_throw();
    repo.langs = langs.json().await.unwrap_throw();
    repo
}
