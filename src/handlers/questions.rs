use std::sync::Arc;

use axum::extract::{Query, State};
use maud::{Markup, html};
use serde::Deserialize;

use crate::{
    logic::{Mode, get_questions, get_questions_by_score, get_score_by_context},
    utils::{AppError, AppState, Htmx, page},
};

#[derive(Deserialize)]
pub struct QuestionsQuery {
    mode: Mode,
    context: String,
}

pub async fn handler(
    htmx: Htmx,
    query: Query<QuestionsQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Markup, AppError> {
    let score = get_score_by_context(&state.extractor, &query.context).await?;
    let questions = get_questions(&query.mode)?;
    let questions = get_questions_by_score(&questions, &score, 50);

    Ok(page(
        htmx,
        html! {
            div class="w-full h-full flex flex-col items-center gap-6" x-data="{index: 0}" {
                @for (i, question) in &mut questions.iter().enumerate() {
                    p
                        class="text-center flex-1 flex items-center text-lg font-medium text-balance"
                        x-show=(format!("index == {}", i))
                    {
                        (&question.question)
                    }
                }
                div class="w-full flex justify-around" {
                    button
                        class="uppercase font-bold flex items-center gap-4 disabled:opacity-40"
                        x-on:click="index -= 1"
                        x-bind:disabled="index === 0"
                    {
                        img src="/static/arrow-left.svg";
                        "Précédente"
                    }
                    button
                        class="uppercase font-bold flex items-center gap-4 disabled:opacity-40"
                        x-on:click="index += 1"
                        x-bind:disabled=(format!("index >= {}", questions.len() - 1))
                    {
                        "Suivante"
                        img src="/static/arrow-right.svg";
                    }
                }
                a
                    class="uppercase font-bold"
                    hx-get="/choices"
                    hx-target="#main"
                    hx-push-url="true"
                {
                    "Retourner au contexte"
                }
            }
        },
    ))
}
