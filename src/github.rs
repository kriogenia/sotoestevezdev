use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Debug, Deserialize)]
pub(super) struct Repository {
    #[serde(rename = "html_url")]
    pub url: String,
    pub language: Option<String>,
    pub forks: i32,
    pub watchers: i32,
    pub size: i32,
    pub topics: Vec<String>,
    pub created_at: String,
}

pub async fn get_repo(repo: impl AsRef<str>) -> Repository {
    let url = format!("https://api.github.com/repos/kriogenia/{}", repo.as_ref());
    Request::get(&url)
        .send()
        .await
        .unwrap_throw()
        .json()
        .await
        .unwrap_throw()
}
