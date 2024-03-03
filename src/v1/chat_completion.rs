use serde::{Deserialize, Serialize};

use crate::v1::common;

#[derive(Debug)]
pub struct ChatCompletionParams {
    pub tools: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub top_p: Option<f32>,
    pub random_seed: Option<u32>,
    pub stream: Option<bool>,
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
            stream: None,
            safe_prompt: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub messages: Vec<ChatCompletionMessage>,
    pub model: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_prompt: Option<bool>,
    // TODO Check this prop (seen in official Python client but not in API doc).
    // pub tool_choice: Option<String>,
    // TODO Check this prop (seen in official Python client but not in API doc).
    // pub response_format: Option<String>,
}
impl ChatCompletionRequest {
    pub fn new(
        model: String,
        messages: Vec<ChatCompletionMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Self {
        let ChatCompletionParams {
            tools,
            temperature,
            max_tokens,
            top_p,
            random_seed,
            stream,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub usage: common::ResponseUsage,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatCompletionChoice {
    pub index: u32,
    pub message: ChatCompletionMessage,
    pub finish_reason: String,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub logprobs: ???
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatCompletionMessage {
    pub role: ChatCompletionMessageRole,
    pub content: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[allow(non_camel_case_types)]
pub enum ChatCompletionMessageRole {
    assistant,
    user,
}
