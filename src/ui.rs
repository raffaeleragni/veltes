use std::time::{SystemTime, UNIX_EPOCH};

use velvet_web::prelude::*;

pub fn app() -> Router {
    Router::new()
        .route("/ui/samples", get(get_all_samples))
        .route("/ui/sample", post(add_new_sample))
        .route("/ui/sample/:id", get(get_one_sample))
        // everything above will be checked by this
        // for how to combine different route groups, check api.rs example
        .authorized_cookie_claims("/ui/fake_login", |claims: Claims| Ok((claims.role == "admin").into()))
        .route("/", get(index))
        .route("/ui/fake_login", get(fake_login))
        .route("/send", get(send))
}

#[derive(Serialize, Deserialize)]
struct Claims {
    role: String,
    exp: u64,
}

#[derive(Clone, Debug, Valuable)]
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

async fn fake_login(jar: CookieJar) -> Result<(CookieJar, Redirect), StatusCode> {
    let jar = CookieToken::set_from_claims(
        jar,
        Claims {
            role: "admin".to_string(),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600,
        },
    )
    .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;
    Ok((jar, Redirect::to("/")))
}

#[instrument(skip(db))]
async fn get_all_samples(
    Extension(db): Extension<Pool<Postgres>>,
) -> Result<SamplesView, AppError> {
    let samples = query_as!(Sample, "select * from sample")
        .fetch_all(&db)
        .await?;
    info!("getting list of samples");
    Ok(SamplesView { samples })
}

#[instrument(skip(db))]
async fn get_one_sample(
    Extension(db): Extension<Pool<Postgres>>,
    Path(id): Path<String>,
) -> Result<SampleView, AppError> {
    let sample = query_as!(Sample, "select * from sample where id = $1", id)
        .fetch_one(&db)
        .await?;
    info!(?sample, value = 123, "getting sample");
    Ok(SampleView { sample })
}

#[instrument(skip(db))]
async fn add_new_sample(
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
async fn index() -> Index {
    Index {}
}

async fn send(Extension(mailer): Extension<MailTransport>) -> AppResult<()> {
    let message = MailMessage::builder()
        .from("test@test.com".parse().unwrap())
        .to("test@test.cim".parse().unwrap())
        .subject("Hi")
        .header(MailContentType::TEXT_PLAIN)
        .body("Hello World".to_string())
        .unwrap();
    mailer.send(&message)?;
    Ok(())
}
