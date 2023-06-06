use std::{borrow::Cow, println};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use semver::Version;
use serde::Deserialize;

/// - <https://docs.docker.com/registry/spec/api/>
/// - <https://docs.docker.com/docker-hub/api/latest/#tag/images/operation/GetNamespacesRepositoriesImagesTags>

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct AuthResponse {
    token: String,
    access_token: String,
    expires_in: u32,
    issued_at: String,
}

async fn fetch_token(image: &str) -> anyhow::Result<AuthResponse> {
    reqwest::get(format!(
        "https://auth.docker.io/token?service=registry.docker.io&scope=repository:{image}:pull"
    ))
    .await?
    .json()
    .await
    .map_err(Into::into)
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct TagResponse {
    name: String,
    tags: Vec<String>,
}

async fn fetch_tags(image: &str, token: &str) -> anyhow::Result<TagResponse> {
    let client = reqwest::Client::new();
    client
        .get(format!("https://registry-1.docker.io/v2/{image}/tags/list"))
        .bearer_auth(token)
        .send()
        .await?
        .json()
        .await
        .map_err(Into::into)
}

fn prepare_image(image: &str) -> Cow<'_, str> {
    if image.as_bytes().contains(&b'/') {
        Cow::Borrowed(image)
    } else {
        tracing::warn!("Incomplete image => Guessing docker library");
        Cow::Owned(format!("library/{image}"))
    }
}

async fn tags_to_versions(tags: Vec<String>) -> Vec<(String, Version)> {
    tokio_rayon::spawn(move || {
        tags.par_iter()
            .cloned()
            .filter_map(|tag| {
                let x = if let Some(tag) = tag.strip_prefix('v') {
                    tag
                } else {
                    &tag
                };

                semver::Version::parse(x).ok().map(|v| (tag, v))
            })
            .collect()
    })
    .await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let image = std::env::args()
        .skip(1)
        .next()
        .expect("Docker image name argument");

    tracing_subscriber::fmt().try_init().unwrap();

    let image = prepare_image(&image);

    let auth = fetch_token(&image).await?;
    let tags = fetch_tags(&image, &auth.token).await?;

    let versions = tags_to_versions(tags.tags).await;
    let mut versions = versions
        .into_iter()
        .filter(|(_, v)| v.pre.is_empty())
        .collect::<Vec<_>>();
    versions.sort_by(|(_, a), (_, b)| b.cmp(a));

    for (tag, _) in versions.into_iter().take(3) {
        println!("{tag}");
    }

    Ok(())
}
