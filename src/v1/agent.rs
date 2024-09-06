use serde::{Deserialize, Serialize};

use crate::v1::{
    chat::{ChatResponseChoice, HasChoices},
    common, constants, tool,
};

// -----------------------------------------------------------------------------
// Definitions

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum AgentMessage {
    Assistant {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<tool::ToolCall>>,
        prefix: bool,
    },
    User {
        content: String,
    },
    Tool {
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },
}
impl AgentMessage {
    pub fn new_assistant_message(content: &str, tool_calls: Option<Vec<tool::ToolCall>>) -> Self {
        Self::Assistant {
            content: content.to_string(),
            tool_calls,
            prefix: false,
        }
    }

    pub fn new_user_message(content: &str) -> Self {
        Self::User {
            content: content.to_string(),
        }
    }
    pub fn new_prefix(content: &str) -> Self {
        Self::Assistant {
            content: content.to_string(),
            tool_calls: None,
            prefix: true,
        }
    }
}

/// The format that the model must output.
///
/// See the [API documentation](https://docs.mistral.ai/api/#operation/createChatCompletion) for more information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub type_: String,
}
impl ResponseFormat {
    pub fn json_object() -> Self {
        Self {
            type_: "json_object".to_string(),
        }
    }
}

// -----------------------------------------------------------------------------
// Request

/// The parameters for the chat request.
///
/// See the [API documentation](https://docs.mistral.ai/api/#operation/createChatCompletion) for more information.
#[derive(Clone, Debug, Serialize)]
pub struct AgentParams {
    /// The maximum number of tokens to generate in the completion.
    ///
    /// Defaults to `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// The minimum number of tokens to generate in the completion.
    ///
    /// Defaults to `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_tokens: Option<u32>,
    /// Stop if one of these tokens is detected.
    ///
    ///Defaults to `None`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    /// The seed to use for random sampling. If set, different calls will generate deterministic results.
    ///
    /// Defaults to `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_seed: Option<u32>,
    /// The format that the model must output.
    ///
    /// Defaults to `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    /// A list of available tools for the model.
    ///
    /// Defaults to `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<tool::Tool>>,
    /// Specifies if/how functions are called.
    ///
    /// Defaults to `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<tool::ToolChoice>,
}
impl Default for AgentParams {
    fn default() -> Self {
        Self {
            max_tokens: None,
            min_tokens: None,
            stop: None,
            random_seed: None,
            response_format: None,
            tools: None,
            tool_choice: None,
        }
    }
}
impl AgentParams {
    pub fn json_default() -> Self {
        Self {
            response_format: Some(ResponseFormat::json_object()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AgentRequest {
    pub messages: Vec<AgentMessage>,
    #[serde(rename = "agent_id")]
    pub id: String,

    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub options: Option<AgentParams>,
    pub stream: bool,
}
impl AgentRequest {
    pub fn new(
        id: &str,
        messages: Vec<AgentMessage>,
        stream: bool,
        options: Option<AgentParams>,
    ) -> Self {
        let id = id.to_string();
        Self {
            messages,
            id,

            stream,
            options,
        }
    }
}

// -----------------------------------------------------------------------------
// Response

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AgentResponse {
    pub id: String,
    pub object: String,
    pub model: constants::Model,
    pub usage: common::ResponseUsage,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub choices: Vec<ChatResponseChoice>,
}
impl HasChoices for AgentResponse {
    fn choices(&self) -> &[ChatResponseChoice] {
        &self.choices
    }
}
