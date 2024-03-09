use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::v1::{chat, common, constants, error};

// -----------------------------------------------------------------------------
// Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatStreamChunk {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub model: constants::Model,
    pub choices: Vec<ChatStreamChunkChoice>,
    pub usage: Option<common::ResponseUsage>,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub logprobs: ???,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatStreamChunkChoice {
    pub index: u32,
    pub delta: ChatStreamChunkChoiceDelta,
    pub finish_reason: Option<String>,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub logprobs: ???,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatStreamChunkChoiceDelta {
    pub role: Option<chat::ChatMessageRole>,
    pub content: String,
}

/// Extracts serialized chunks from a stream message.
pub fn get_chunk_from_stream_message_line(
    line: &str,
) -> Result<Option<Vec<ChatStreamChunk>>, error::ApiError> {
    if line.trim() == "data: [DONE]" {
        return Ok(None);
    }

    let chunk_as_json = line.trim_start_matches("data: ").trim();
    if chunk_as_json.is_empty() {
        return Ok(Some(vec![]));
    }

    // Attempt to deserialize the JSON string into ChatStreamChunk
    match from_str::<ChatStreamChunk>(chunk_as_json) {
        Ok(chunk) => Ok(Some(vec![chunk])),
        Err(e) => Err(error::ApiError {
            message: e.to_string(),
        }),
    }
}
