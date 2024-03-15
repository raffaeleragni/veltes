mod app;
use app::*;

use velvet::prelude::*;

#[tokio::main]
async fn main() {
    #[derive(RustEmbed)]
    #[folder = "statics"]
    struct S;

    let db = database().await;
    sqlx::migrate!().run(&db).await.unwrap();

    let r = Router::new()
        .route("/", get(index))
        .route("/ui/samples", get(get_all_samples))
        .route("/ui/sample", post(add_new_sample))
        .route("/ui/sample/:id", get(get_one_sample));
    
    App::new()
        .router(r)
        .inject(db)
        .statics::<S>()
        .start()
        .await
        .unwrap();
}
