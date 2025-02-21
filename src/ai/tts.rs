use serde::{Deserialize, Serialize};

use super::huggingface::HuggingFaceModel;

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "id", content = "model")]
pub(crate) enum TtsProvider {
    HuggingFace(HuggingFaceModel),
}
