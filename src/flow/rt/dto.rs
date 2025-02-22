use std::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::{flow::subflow::dto::NextActionType, variable::dto::SimpleVariable};

#[derive(Deserialize, PartialEq, Eq)]
pub(crate) enum UserInputResult {
    Successful,
    Timeout,
}

#[derive(Deserialize)]
pub(crate) struct Request {
    #[serde(rename = "robotId")]
    pub(crate) robot_id: String,
    #[serde(rename = "mainFlowId")]
    pub(crate) main_flow_id: String,
    #[serde(rename = "sessionId")]
    pub(crate) session_id: Option<String>,
    #[serde(rename = "userInputResult")]
    pub(crate) user_input_result: UserInputResult,
    #[serde(rename = "userInput")]
    pub(crate) user_input: String,
    #[serde(rename = "importVariables")]
    pub(crate) import_variables: Option<Vec<SimpleVariable>>,
    #[serde(rename = "userInputIntent")]
    pub(crate) user_input_intent: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct CollectData {
    #[serde(rename = "varName")]
    pub(crate) var_name: String,
    pub(crate) value: String,
}

#[derive(Clone, Deserialize, Serialize, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(compare(PartialEq))]
pub(crate) enum AnswerContentType {
    TextPlain,
    TextHtml,
}

#[derive(Serialize)]
pub(crate) struct AnswerData {
    pub(crate) content: String,
    #[serde(rename = "contentType")]
    pub(crate) content_type: AnswerContentType,
}

#[derive(Serialize)]
pub(crate) struct Response {
    #[serde(rename = "sessionId")]
    pub(crate) session_id: String,
    pub(crate) have_answers: bool,
    pub(crate) answers: Vec<AnswerData>,
    #[serde(rename = "collectData")]
    pub(crate) collect_data: Vec<CollectData>,
    #[serde(rename = "nextAction")]
    pub(crate) next_action: NextActionType,
    #[serde(rename = "extraData")]
    pub(crate) extra_data: ExtraData,
    #[serde(rename = "sseReceiverTicket")]
    pub(crate) sse_receiver_ticket: String,
}

impl Response {
    pub(crate) fn new(req: &Request) -> Self {
        Self {
            session_id: req.session_id.as_ref().unwrap().clone(),
            have_answers: false,
            answers: Vec::with_capacity(5),
            collect_data: Vec::with_capacity(10),
            next_action: NextActionType::None,
            extra_data: ExtraData {
                external_link: String::new(),
            },
            sse_receiver_ticket: String::new(),
        }
    }
}

#[derive(Serialize)]
pub(crate) struct ExtraData {
    #[serde(rename = "externalLink")]
    pub(crate) external_link: String,
}
