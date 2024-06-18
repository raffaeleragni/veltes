use axum_test::TestServer;
use std::time::Duration;
use veltes::api;
use velvet_web::prelude::*;

pub async fn create_server() -> TestServer {
    JWT::JwkUrls.setup().await.unwrap();
    let db = postgres().await;
    TestServer::new(api::app().layer(Extension(db))).unwrap()
}

pub struct Provider<'a> {
    url: &'a str,
    client: &'a str,
    client_secret: Option<&'a str>,
}

pub struct User<'a>(&'a str, &'a str);

pub const PRIMARY_MASTER: Provider = Provider {
    url: "http://localhost:8888/realms/master/protocol/openid-connect/token",
    client: "admin-cli",
    client_secret: None,
};

pub const ADMIN: User = User("admin", "admin");

pub async fn login(provider: &Provider<'_>, user: &User<'_>) -> String {
    #[derive(Serialize)]
    struct LoginForm<'a> {
        grant_type: &'a str,
        username: &'a str,
        password: &'a str,
        client_id: &'a str,
        client_secret: Option<&'a str>,
    }
    #[derive(Deserialize)]
    struct Response {
        access_token: String,
    }
    let form = LoginForm {
        grant_type: "password",
        username: user.0,
        password: user.1,
        client_id: provider.client,
        client_secret: provider.client_secret,
    };
    client()
        .post(provider.url)
        .timeout(Duration::from_millis(1000))
        .form(&form)
        .send()
        .await
        .unwrap()
        .json::<Response>()
        .await
        .unwrap()
        .access_token
}
