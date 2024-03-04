use serde::{Deserialize, Serialize};

use crate::v1::{common, constants};

#[derive(Debug)]
pub struct EmbeddingRequestOptions {
    pub encoding_format: Option<EmbeddingRequestEncodingFormat>,
}
impl Default for EmbeddingRequestOptions {
    fn default() -> Self {
        Self {
            encoding_format: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    pub model: constants::EmbedModel,
    pub input: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<EmbeddingRequestEncodingFormat>,
}
impl EmbeddingRequest {
    pub fn new(
        model: constants::EmbedModel,
        input: Vec<String>,
        options: Option<EmbeddingRequestOptions>,
    ) -> Self {
        let EmbeddingRequestOptions { encoding_format } = options.unwrap_or_default();

        Self {
            model,
            input,
            encoding_format,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum EmbeddingRequestEncodingFormat {
    float,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbeddingResponse {
    pub id: String,
    pub object: String,
    pub model: constants::EmbedModel,
    pub data: Vec<EmbeddingResponseDataItem>,
    pub usage: common::ResponseUsage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmbeddingResponseDataItem {
    pub index: u32,
    pub embedding: Vec<f32>,
    pub object: String,
}
