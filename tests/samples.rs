mod testing;

use axum::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use std::{collections::HashSet, str::FromStr};
use testing::*;
use veltes::app;
use velvet_web::prelude::*;

#[tokio::test]
async fn test_gives_empty() {
    let mut server = app::app().await.unwrap().as_test_server().await;
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
