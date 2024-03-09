use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{any::Any, collections::HashMap};

// -----------------------------------------------------------------------------
// Definitions

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ToolCall {
    pub function: ToolCallFunction,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct ToolCallFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tool {
    pub r#type: ToolType,
    pub function: ToolFunction,
}
impl Tool {
    pub fn new(
        function_name: String,
        function_description: String,
        function_parameters: Vec<ToolFunctionParameter>,
    ) -> Self {
        let properties: HashMap<String, ToolFunctionParameterProperty> = function_parameters
            .into_iter()
            .map(|param| {
                (
                    param.name,
                    ToolFunctionParameterProperty {
                        r#type: param.r#type,
                        description: param.description,
                    },
                )
            })
            .collect();
        let property_names = properties.keys().cloned().collect();

        let parameters = ToolFunctionParameters {
            r#type: ToolFunctionParametersType::Object,
            properties,
            required: property_names,
        };

        Self {
            r#type: ToolType::Function,
            function: ToolFunction {
                name: function_name,
                description: function_description,
                parameters,
            },
        }
    }
}

// -----------------------------------------------------------------------------
// Request

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunction {
    name: String,
    description: String,
    parameters: ToolFunctionParameters,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunctionParameter {
    name: String,
    description: String,
    r#type: ToolFunctionParameterType,
}
impl ToolFunctionParameter {
    pub fn new(name: String, description: String, r#type: ToolFunctionParameterType) -> Self {
        Self {
            name,
            r#type,
            description,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunctionParameters {
    r#type: ToolFunctionParametersType,
    properties: HashMap<String, ToolFunctionParameterProperty>,
    required: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ToolFunctionParameterProperty {
    r#type: ToolFunctionParameterType,
    description: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolFunctionParametersType {
    #[serde(rename = "object")]
    Object,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolFunctionParameterType {
    #[serde(rename = "string")]
    String,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum ToolChoice {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
}

// -----------------------------------------------------------------------------
// Custom

#[async_trait]
pub trait Function {
    async fn execute(&self, arguments: String) -> Box<dyn Any + Send>;
}
