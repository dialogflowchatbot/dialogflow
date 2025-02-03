use std::vec::Vec;

use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// pub(crate) struct QuestionAnswerData {
//     pub(super) id: Option<String>,
//     #[serde(rename = "qaData")]
//     pub(super) qa_data: QuestionAnswerPair,
// }

#[derive(Deserialize, Serialize)]
pub(crate) struct QuestionAnswerPair {
    pub(super) id: Option<String>,
    pub(super) question: QuestionData,
    #[serde(rename = "similarQuestions")]
    pub(super) similar_questions: Vec<QuestionData>,
    pub(crate) answer: String,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct QuestionData {
    pub(super) question: String,
    pub(super) vec_row_id: Option<i64>,
}

#[derive(Serialize, sqlx::FromRow)]
pub(crate) struct DocData {
    pub(crate) id: i64,
    #[serde(rename = "fileName")]
    pub(crate) file_name: String,
    #[serde(rename = "fileSize")]
    pub(crate) file_size: i64,
    #[serde(rename = "docContent")]
    pub(crate) doc_content: String,
}
