mod api;
mod proxy;
mod ui;

use velvet_web::prelude::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    #[derive(RustEmbed)]
    #[folder = "statics"]
    struct S;

    JWT::JwkUrls.setup().await?;

    let db = postgres().await;
    sqlx::migrate!().run(&db).await?;

    App::new()
        .router(ui::app())
        .router(api::app())
        .router(proxy::app())
        .inject(db)
        .inject(client())
        .statics::<S>()
        .start()
        .await;
    Ok(())
}
