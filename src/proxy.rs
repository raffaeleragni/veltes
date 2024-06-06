use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new().route("/proxy", get(proxy))
}

#[derive(Deserialize, Serialize)]
struct Sample {
    id: String,
    name: String,
}

async fn proxy(Extension(client): Extension<Client>) -> AppResult<Json<Vec<Sample>>> {
    let result = client
        .get("http://localhost:8080/api/sample")
        .send()
        .await?
        .json::<Vec<Sample>>()
        .await?;
    Ok(Json(result))
}
