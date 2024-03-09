use serde::{Deserialize, Serialize};

use crate::v1::{common, constants, tool};

// -----------------------------------------------------------------------------
// Definitions

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: ChatMessageRole,
    pub content: String,
    pub tool_calls: Option<Vec<tool::ToolCall>>,
}
impl ChatMessage {
    pub fn new_assistant_message(content: &str, tool_calls: Option<Vec<tool::ToolCall>>) -> Self {
        Self {
            role: ChatMessageRole::Assistant,
            content: content.to_string(),
            tool_calls,
        }
    }

    pub fn new_user_message(content: &str) -> Self {
        Self {
            role: ChatMessageRole::User,
            content: content.to_string(),
            tool_calls: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChatMessageRole {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

// -----------------------------------------------------------------------------
// Request

#[derive(Debug)]
pub struct ChatParams {
    pub max_tokens: Option<u32>,
    pub random_seed: Option<u32>,
    pub safe_prompt: Option<bool>,
    pub temperature: Option<f32>,
    pub tool_choice: Option<tool::ToolChoice>,
    pub tools: Option<Vec<tool::Tool>>,
    pub top_p: Option<f32>,
}
impl Default for ChatParams {
    fn default() -> Self {
        Self {
            max_tokens: None,
            random_seed: None,
            safe_prompt: None,
            temperature: None,
            tool_choice: None,
            tools: None,
            top_p: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
    pub model: constants::Model,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_prompt: Option<bool>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<tool::ToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<tool::Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    // TODO Check this prop (seen in official Python client but not in API doc).
    // pub tool_choice: Option<String>,
    // TODO Check this prop (seen in official Python client but not in API doc).
    // pub response_format: Option<String>,
}
impl ChatRequest {
    pub fn new(
        model: constants::Model,
        messages: Vec<ChatMessage>,
        stream: bool,
        options: Option<ChatParams>,
    ) -> Self {
        let ChatParams {
            max_tokens,
            random_seed,
            safe_prompt,
            temperature,
            tool_choice,
            tools,
            top_p,
        } = options.unwrap_or_default();

        Self {
            messages,
            model,

            max_tokens,
            random_seed,
            safe_prompt,
            stream,
            temperature,
            tool_choice,
            tools,
            top_p,
        }
    }
}

// -----------------------------------------------------------------------------
// Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub model: constants::Model,
    pub choices: Vec<ChatResponseChoice>,
    pub usage: common::ResponseUsage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatResponseChoice {
    pub index: u32,
    pub message: ChatMessage,
    pub finish_reason: ChatResponseChoiceFinishReason,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub logprobs: ???
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ChatResponseChoiceFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
