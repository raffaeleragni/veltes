use velvet::prelude::*;

use sqlx::{query, query_as};
use tracing::instrument;

pub fn app() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/ui/samples", get(get_all_samples))
        .route("/ui/sample", post(add_new_sample))
        .route("/ui/sample/:id", get(get_one_sample))
}

pub struct Sample {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct NewSample {
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

#[derive(Template)]
#[template(path = "samples.html")]
pub struct SamplesView {
    samples: Vec<Sample>,
}

#[derive(Template)]
#[template(path = "sample.html")]
pub struct SampleView {
    sample: Sample,
}

#[instrument(skip(db))]
pub async fn get_all_samples(
    Extension(db): Extension<Pool<Postgres>>,
) -> Result<SamplesView, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    info!("getting list of samples");
    Ok(SamplesView { samples })
}

#[instrument(skip(db))]
pub async fn get_one_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<SampleView, AppError> {
    let sample = query_as!(Sample, "select * from sample where id = $1", id)
        .fetch_one(&db)
        .await?;
    info!("getting sample");
    Ok(SampleView { sample })
}

#[instrument(skip(db))]
pub async fn add_new_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Form(new): Form<NewSample>,
) -> Result<SamplesView, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    query!(
        "insert into sample (id, name) values ($1, $2)",
        id,
        new.name
    )
    .execute(&db)
    .await?;
    info!("sample added");
    get_all_samples(Extension(db)).await
}

#[instrument]
pub async fn index() -> Index {
    Index {}
}