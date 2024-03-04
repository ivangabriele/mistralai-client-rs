use serde::{Deserialize, Serialize};

use crate::v1::{common, constants};

// -----------------------------------------------------------------------------
// Definitions

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: ChatMessageRole,
    pub content: String,
}

#[derive(Clone, Debug, strum_macros::Display, Eq, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum ChatMessageRole {
    assistant,
    user,
}

// -----------------------------------------------------------------------------
// Request

#[derive(Debug)]
pub struct ChatCompletionParams {
    pub tools: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub random_seed: Option<u32>,
    pub safe_prompt: Option<bool>,
}
impl Default for ChatCompletionParams {
    fn default() -> Self {
        Self {
            tools: None,
            temperature: None,
            max_tokens: None,
            top_p: None,
            random_seed: None,
            safe_prompt: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<ChatMessage>,
    pub model: constants::Model,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u32>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_prompt: Option<bool>,
    // TODO Check this prop (seen in official Python client but not in API doc).
    // pub tool_choice: Option<String>,
    // TODO Check this prop (seen in official Python client but not in API doc).
    // pub response_format: Option<String>,
}
impl ChatCompletionRequest {
    pub fn new(
        model: constants::Model,
        messages: Vec<ChatMessage>,
        stream: bool,
        options: Option<ChatCompletionParams>,
    ) -> Self {
        let ChatCompletionParams {
            tools,
            temperature,
            max_tokens,
            top_p,
            random_seed,
            safe_prompt,
        } = options.unwrap_or_default();

        Self {
            messages,
            model,
            tools,
            temperature,
            max_tokens,
            top_p,
            random_seed,
            stream,
            safe_prompt,
        }
    }
}

// -----------------------------------------------------------------------------
// Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub model: constants::Model,
    pub choices: Vec<ChatCompletionResponseChoice>,
    pub usage: common::ResponseUsage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatCompletionResponseChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: String,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub logprobs: ???
}

// -----------------------------------------------------------------------------
// Stream

#[derive(Debug, Deserialize)]
pub struct ChatCompletionStreamChunk {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub model: constants::Model,
    pub choices: Vec<ChatCompletionStreamChunkChoice>,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub usage: ???,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionStreamChunkChoice {
    pub index: u32,
    pub delta: ChatCompletionStreamChunkChoiceDelta,
    pub finish_reason: Option<String>,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub logprobs: ???,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionStreamChunkChoiceDelta {
    pub role: Option<ChatMessageRole>,
    pub content: String,
}
