use crate::v1::error::ApiError;
use reqwest::Error as ReqwestError;

use crate::v1::{
    chat_completion::{
        ChatCompletionMessage, ChatCompletionParams, ChatCompletionRequest, ChatCompletionResponse,
    },
    constants::{EmbedModel, Model, API_URL_BASE},
    embedding::{EmbeddingRequest, EmbeddingRequestOptions, EmbeddingResponse},
    error::ClientError,
    model_list::ModelListResponse,
};

pub struct Client {
    pub api_key: String,
    pub endpoint: String,
    pub max_retries: u32,
    pub timeout: u32,
}

impl Client {
    pub fn new(
        api_key: Option<String>,
        endpoint: Option<String>,
        max_retries: Option<u32>,
        timeout: Option<u32>,
    ) -> Result<Self, ClientError> {
        let api_key = api_key.unwrap_or(match std::env::var("MISTRAL_API_KEY") {
            Ok(api_key_from_env) => api_key_from_env,
            Err(_) => return Err(ClientError::ApiKeyError),
        });
        let endpoint = endpoint.unwrap_or(API_URL_BASE.to_string());
        let max_retries = max_retries.unwrap_or(5);
        let timeout = timeout.unwrap_or(120);

        Ok(Self {
            api_key,
            endpoint,
            max_retries,
            timeout,
        })
    }

    pub fn chat(
        &self,
        model: Model,
        messages: Vec<ChatCompletionMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        let request = ChatCompletionRequest::new(model, messages, options);

        let response = self.post_sync("/chat/completions", &request)?;
        let result = response.json::<ChatCompletionResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub async fn chat_async(
        &self,
        model: Model,
        messages: Vec<ChatCompletionMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        let request = ChatCompletionRequest::new(model, messages, options);

        let response = self.post_async("/chat/completions", &request).await?;
        let result = response.json::<ChatCompletionResponse>().await;
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub fn embeddings(
        &self,
        model: EmbedModel,
        input: Vec<String>,
        options: Option<EmbeddingRequestOptions>,
    ) -> Result<EmbeddingResponse, ApiError> {
        let request = EmbeddingRequest::new(model, input, options);

        let response = self.post_sync("/embeddings", &request)?;
        let result = response.json::<EmbeddingResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub fn list_models(&self) -> Result<ModelListResponse, ApiError> {
        let response = self.get_sync("/models")?;
        let result = response.json::<ModelListResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub async fn list_models_async(&self) -> Result<ModelListResponse, ApiError> {
        let response = self.get_async("/models").await?;
        let result = response.json::<ModelListResponse>().await;
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    fn build_request_sync(
        &self,
        request: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        let user_agent = format!(
            "ivangabriele/mistralai-client-rs/{}",
            env!("CARGO_PKG_VERSION")
        );

        let request_builder = request
            .bearer_auth(&self.api_key)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", user_agent);

        request_builder
    }

    fn build_request_async(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let user_agent = format!(
            "ivangabriele/mistralai-client-rs/{}",
            env!("CARGO_PKG_VERSION")
        );

        let request_builder = request
            .bearer_auth(&self.api_key)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", user_agent);

        request_builder
    }

    fn get_sync(&self, path: &str) -> Result<reqwest::blocking::Response, ApiError> {
        let client_sync = reqwest::blocking::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        let request = self.build_request_sync(client_sync.get(url));

        let result = request.send();
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let status = response.status();
                    let text = response.text().unwrap();
                    Err(ApiError {
                        message: format!("{}: {}", status, text),
                    })
                }
            }
            Err(error) => Err(ApiError {
                message: error.to_string(),
            }),
        }
    }

    async fn get_async(&self, path: &str) -> Result<reqwest::Response, ApiError> {
        let reqwest_client = reqwest::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        let request_builder = reqwest_client.get(url);
        let request = self.build_request_async(request_builder);

        let result = request.send().await.map_err(|e| self.to_api_error(e));
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ApiError {
                        message: format!("{}: {}", status, text),
                    })
                }
            }
            Err(error) => Err(ApiError {
                message: error.to_string(),
            }),
        }
    }

    fn post_sync<T: serde::ser::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<reqwest::blocking::Response, ApiError> {
        let reqwest_client = reqwest::blocking::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        let request_builder = reqwest_client.post(url).json(params);
        let request = self.build_request_sync(request_builder);

        let result = request.send();
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let status = response.status();
                    let text = response.text().unwrap_or_default();
                    Err(ApiError {
                        message: format!("{}: {}", status, text),
                    })
                }
            }
            Err(error) => Err(ApiError {
                message: error.to_string(),
            }),
        }
    }

    async fn post_async<T: serde::ser::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<reqwest::Response, ApiError> {
        let reqwest_client = reqwest::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        let request_builder = reqwest_client.post(url).json(params);
        let request = self.build_request_async(request_builder);

        let result = request.send().await.map_err(|e| self.to_api_error(e));
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    Err(ApiError {
                        message: format!("{}: {}", status, text),
                    })
                }
            }
            Err(error) => Err(ApiError {
                message: error.to_string(),
            }),
        }
    }

    fn to_api_error(&self, err: ReqwestError) -> ApiError {
        ApiError {
            message: err.to_string(),
        }
    }
}
