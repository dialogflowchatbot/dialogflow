use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct MainFlowDetail {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) enabled: bool,
}
