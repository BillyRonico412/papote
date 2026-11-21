use derive_more::Display;
use rig::extractor::Extractor;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub mod ai;

#[derive(Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Amis,
    Couple,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Question(pub String);

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Questions {
    pub questions: Vec<Question>,
}

pub async fn get_questions(
    extractor: &Extractor<Questions>,
    mode: &Mode,
    context: &str,
) -> anyhow::Result<Vec<Question>> {
    let text = format!("Mode: {}\nContext: {}", mode, context);
    Ok(extractor.extract(text).await?.output.questions)
}
