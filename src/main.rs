mod api;
mod app;
mod proxy;
mod ui;

use velvet_web::prelude::*;

#[tokio::main]
async fn main() -> AppResult<()> {
    app::app().await?.start().await
}
