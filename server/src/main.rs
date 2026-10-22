use axum::{Json, Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

use crate::logic::{
    Mode, Question, Score, ai::create_extractor, get_questions, get_questions_by_score,
    get_score_by_context,
};
mod logic;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    let app = Router::new()
        .route("/api/questions", get(questions))
        .fallback_service(ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server running in http://localhost:8080");
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Deserialize)]
pub struct QuestionsQuery {
    mode: Mode,
    nb_questions: Option<usize>,
    context: Option<String>,
}

#[derive(Serialize)]
pub struct QuestionsResult {
    score: Score,
    questions: Vec<Question>,
}

pub async fn questions(
    Query(questions_query): Query<QuestionsQuery>,
) -> Result<Json<QuestionsResult>, AppError> {
    let extractor = create_extractor().await?;
    let score =
        get_score_by_context(extractor, &questions_query.context.unwrap_or_default()).await?;
    let questions = get_questions(&questions_query.mode)?;
    let questions_by_score = get_questions_by_score(
        &questions,
        &score,
        questions_query.nb_questions.unwrap_or(5),
    );
    Ok(Json(QuestionsResult {
        questions: questions_by_score,
        score,
    }))
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        println!("{:?}", self.0);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
