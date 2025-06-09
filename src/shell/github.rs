use crate::{github::get_user, icons::Icon};

pub(super) async fn run() -> Vec<String> {
    let user = get_user().await;
    let link = format!(r#"<a href="{}">{}</a>"#, user.url, user.url);
    vec![
        format!(
            r"{} <strong>{}</strong> ({})",
            Icon::GitHub,
            user.login,
            link
        ),
        format!(
            r#"   {} <span class="tags">{}</span>"#,
            user.location, user.company
        ),
        String::new(),
        format!("  ❯ <em>Public repos:</em>     {}", user.public_repos),
        format!("  ❯ <em>Public gists:</em>     {}", user.public_gists),
        format!("  ❯ <em>Followers:</em>        {}", user.followers),
        format!("  ❯ <em>Following:</em>        {}", user.following),
        format!(
            "  ❯ <em>Created:</em>  {}",
            user.created_at.split('T').next().unwrap_or_default()
        ),
    ]
}
