use std::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum Protocol {
    HTTP,
    HTTPS,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum Method {
    GET,
    POST,
}

#[derive(Clone, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) enum PostContentType {
    UrlEncoded,
    JSON,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) enum ValueSource {
    Val,
    Var,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct HttpReqParam {
    pub(crate) name: String,
    pub(crate) value: String,
    #[serde(rename = "valueSource")]
    pub(crate) value_source: ValueSource,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct HttpReqInfo {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) protocol: Protocol,
    pub(crate) method: Method,
    pub(crate) address: String,
    #[serde(rename = "timeoutMilliseconds")]
    pub(crate) timeout_milliseconds: u64,
    #[serde(rename = "postContentType")]
    pub(crate) post_content_type: PostContentType,
    pub(crate) headers: Vec<HttpReqParam>,
    #[serde(rename = "queryParams")]
    pub(crate) query_params: Vec<HttpReqParam>,
    #[serde(rename = "formData")]
    pub(crate) form_data: Vec<HttpReqParam>,
    #[serde(rename = "requestBody")]
    pub(crate) request_body: String,
    #[serde(rename = "userAgent")]
    pub(crate) user_agent: String,
    #[serde(rename = "asyncReq")]
    pub(crate) async_req: bool,
}

pub(crate) enum ResponseData {
    Str(String),
    Bin(Vec<u8>),
    None,
}
