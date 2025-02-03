use serde::{Deserialize, Serialize};

use super::huggingface::{load_bert_model_files, HuggingFaceModel, HuggingFaceModelInfo};

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "id", content = "model")]
pub(crate) enum AsrProvider {
    HuggingFace(HuggingFaceModel),
}
