use velvet_web::prelude::*;

pub async fn app() -> AppResult<App> {
    #[derive(RustEmbed)]
    #[folder = "statics"]
    struct S;

    JWT::JwkUrls.setup().await?;

    let db = postgres().await;
    sqlx::migrate!().run(&db).await?;

    Ok(App::new()
        .router(crate::ui::app())
        .router(crate::api::app())
        .router(crate::proxy::app())
        .inject(db)
        .inject(client())
        .inject(mailer())
        .statics::<S>())
}
