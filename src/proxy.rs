use velvet::prelude::*;

pub fn app() -> Router {
    Router::new().route("/github", get(github_proxy_sample))
}

async fn github_proxy_sample(Extension(client): Extension<Client>) -> AppResult<String> {
    Ok(client
        .get("https://github.com/")
        .send()
        .await?
        .text()
        .await?)
}
