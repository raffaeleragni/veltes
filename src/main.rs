use velvet::prelude::*;

use sqlx::{query, query_as};
use tracing::instrument;

struct Sample {
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct NewSample {
    name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[derive(Template)]
#[template(path = "samples.html")]
struct SamplesView {
    samples: Vec<Sample>,
}

#[derive(Template)]
#[template(path = "sample.html")]
struct SampleView {
    sample: Sample,
}

#[instrument(skip(db))]
async fn get_samples(Extension(db): Extension<Pool<Postgres>>) -> Result<SamplesView, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    Ok(SamplesView { samples })
}

#[instrument(skip(db))]
async fn get_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<SampleView, AppError> {
    let sample = query_as!(Sample, "select * from sample where id = $1", id)
        .fetch_one(&db)
        .await?;
    Ok(SampleView { sample })
}

#[instrument(skip(db))]
async fn add_sample(
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
    info!("span created");
    get_samples(Extension(db)).await
}

#[instrument]
async fn index() -> Index {
    Index {}
}

#[derive(RustEmbed)]
#[folder = "statics"]
struct Asset;

#[tokio::main]
async fn main() {
    let app = App::new(
        Router::new()
            .route("/", get(index))
            .route("/ui/samples", get(get_samples))
            .route("/ui/sample", get(get_sample).post(add_sample)),
    );
    let db = database().await;
    sqlx::migrate!().run(&db).await.unwrap();
    app.inject(db)
        .include_static::<Asset>("text/css", "/styles.css", "styles.css")
        .include_static::<Asset>("application/javascript", "/htmx.min.js", "htmx.min.js")
        .start()
        .await
        .unwrap();
}
