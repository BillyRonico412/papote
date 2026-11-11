use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;

use crate::{logic::ai::create_extractor, utils::AppState};

mod handlers;
mod logic;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let extractor = create_extractor().await?;
    let state = Arc::new(AppState { extractor });

    let app = Router::new()
        .route("/", get(handlers::index::handler))
        .route("/choices", get(handlers::choices::handler))
        .route("/context/{mode}", get(handlers::context::handler))
        .route("/menu", get(handlers::menu::handler))
        .route("/principe", get(handlers::principe::handler))
        .route("/a-propos", get(handlers::about::handler))
        .route("/questions", get(handlers::questions::handler))
        .with_state(state)
        .nest_service("/static", ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running in http://localhost:8080");
    axum::serve(listener, app).await?;
    Ok(())
}
