use axum::{Router, http::StatusCode, response::IntoResponse};
use tower_http::services::ServeDir;

mod handlers;
mod logic;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let app = Router::new().fallback_service(ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running in http://localhost:8080");
    axum::serve(listener, app).await?;
    Ok(())
}

pub enum AppError {
    Anyhow(anyhow::Error),
    BadRequest,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Anyhow(e) => {
                println!("{:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            AppError::BadRequest => StatusCode::BAD_REQUEST.into_response(),
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Anyhow(err.into())
    }
}
