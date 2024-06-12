use std::{collections::HashSet, str::FromStr, time::Duration};

use axum::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use veltes::api;
use velvet_web::prelude::*;

#[tokio::test]
async fn test_gives_empty() {
    let mut server = create_server().await;
    let expected = HashSet::new();
    server
        .assert_samples(&PRIMARY_MASTER, &ADMIN, &expected)
        .await;
}

#[derive(PartialEq, Hash, Eq, Debug, Deserialize)]
struct Sample {
    id: String,
    name: String,
}

trait TestSamples {
    #[allow(clippy::ptr_arg)]
    async fn assert_samples(
        &mut self,
        provider: &Provider<'_>,
        user: &User<'_>,
        expected: &HashSet<Sample>,
    );
}

impl TestSamples for TestServer {
    async fn assert_samples(
        &mut self,
        provider: &Provider<'_>,
        user: &User<'_>,
        expected: &HashSet<Sample>,
    ) {
        let header_value = format!("Bearer {}", login(provider, user).await);
        self.add_header(
            HeaderName::from_str("Authorization").unwrap(),
            HeaderValue::from_str(&header_value).unwrap(),
        );
        let test = self.get("/api/sample");
        test.await.assert_json::<HashSet<Sample>>(expected);
    }
}

async fn create_server() -> TestServer {
    JWT::JwkUrls.setup().await.unwrap();
    let db = postgres().await;
    TestServer::new(api::app().layer(Extension(db))).unwrap()
}

struct Provider<'a> {
    url: &'a str,
    client: &'a str,
    client_secret: Option<&'a str>,
}

struct User<'a>(&'a str, &'a str);

const PRIMARY_MASTER: Provider = Provider {
    url: "http://localhost:8888/realms/master/protocol/openid-connect/token",
    client: "admin-cli",
    client_secret: None,
};

const ADMIN: User = User("admin", "admin");

async fn login(provider: &Provider<'_>, user: &User<'_>) -> String {
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
