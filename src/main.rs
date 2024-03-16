mod api;
mod ui;

use velvet::prelude::*;

#[tokio::main]
async fn main() {
    #[derive(RustEmbed)]
    #[folder = "statics"]
    struct S;

    let db = database().await;
    sqlx::migrate!().run(&db).await.unwrap();

    App::new()
        .router(ui::app())
        .router(api::app())
        .inject(db)
        .statics::<S>()
        .start()
        .await;
}
