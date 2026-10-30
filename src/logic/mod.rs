use rig::{extractor::Extractor, serde_json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::BufReader};
pub mod ai;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Amis,
    Couple,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Amis => "Amis",
                Mode::Couple => "Couple",
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct Evaluations {
    pub intimite: u32,
    pub piquant: u32,
    pub humour: u32,
    pub energie: u32,
    pub proximite: u32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Forme {
    Recit,
    Opinion,
    Dilemme,
    Scenario,
    Portrait,
    Deviner,
    Designer,
    Aveu,
    Demande,
    Defi,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Sujet {
    Lien,
    Amour,
    Sexualite,
    Entourage,
    Jeunesse,
    Emotions,
    Valeurs,
    Avenir,
    Transgressions,
    Quotidien,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Score {
    pub evaluation: Evaluations,
    pub sujets: Vec<Sujet>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: String,
    pub question: String,
    pub evaluations: Evaluations,
    pub sujet: Sujet,
    pub forme: Forme,
}

impl Question {
    fn compute_score(&self, score: &Score) -> f32 {
        ((score.evaluation.intimite as f32 - self.evaluations.intimite as f32).powi(2)
            + (score.evaluation.energie as f32 - self.evaluations.energie as f32).powi(2)
            + (score.evaluation.humour as f32 - self.evaluations.humour as f32).powi(2)
            + (score.evaluation.piquant as f32 - self.evaluations.piquant as f32).powi(2)
            + (score.evaluation.proximite as f32 - self.evaluations.proximite as f32).powi(2))
        .sqrt()
            + rand::random_range(0..3) as f32
    }
}

pub async fn get_score_by_context(
    extractor: Extractor<Score>,
    context: &str,
) -> anyhow::Result<Score> {
    Ok(extractor.extract(context).await?.output)
}

pub fn get_questions(mode: &Mode) -> anyhow::Result<Vec<Question>> {
    let path = match mode {
        Mode::Amis => "./generated/amis.json",
        Mode::Couple => "./generated/couple.json",
    };
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let questions: Vec<Question> = serde_json::from_reader(reader)?;
    Ok(questions)
}

pub fn get_questions_by_score(
    questions: &Vec<Question>,
    score: &Score,
    nb_question: usize,
) -> Vec<Question> {
    let mut questions_index_score = questions
        .iter()
        .filter(|question| score.sujets.contains(&question.sujet))
        .map(|question| question.compute_score(score))
        .enumerate()
        .collect::<Vec<_>>();
    questions_index_score.sort_by(|(_, score_a), (_, score_b)| score_a.total_cmp(score_b));
    questions_index_score
        .iter()
        .take(nb_question)
        .map(|(index, _)| questions[*index].clone())
        .collect::<Vec<_>>()
}
