use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new()
        .route("/api/sample", get(get_samples).post(new_sample))
        .route("/api/sample/:id", get(get_sample))
        .authorized_bearer(|token| Ok(claims_for::<Claims>(token)?.role == "admin"))
}

#[derive(Deserialize)]
struct Claims {
    role: String,
}

#[derive(Serialize)]
struct Sample {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct NewSample {
    name: String,
}

#[instrument(skip(db))]
async fn get_samples(
    Extension(db): Extension<Pool<Postgres>>,
) -> Result<Json<Vec<Sample>>, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    info!("returing all samples");
    Ok(Json(samples))
}

#[instrument(skip(db))]
async fn get_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<Json<Sample>, AppError> {
    let sample = query_as!(Sample, "select * from sample where id = $1", id)
        .fetch_one(&db)
        .await?;
    info!("returning sample");
    Ok(Json(sample))
}

#[instrument(skip(db))]
async fn new_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Json(sample): Json<NewSample>,
) -> Result<Json<Sample>, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    query!(
        "insert into sample (id, name) values($1, $2)",
        id,
        sample.name
    )
    .execute(&db)
    .await?;
    info!("sample created");
    get_sample(Extension(db), Path(id)).await
}
