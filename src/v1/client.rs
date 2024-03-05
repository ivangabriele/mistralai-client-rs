use futures::stream::StreamExt;
use futures::Stream;
use reqwest::Error as ReqwestError;
use serde_json::from_str;

use crate::v1::error::ApiError;

use crate::v1::{
    chat_completion::{
        ChatCompletionParams, ChatCompletionRequest, ChatCompletionResponse, ChatMessage,
    },
    constants::{EmbedModel, Model, API_URL_BASE},
    embedding::{EmbeddingRequest, EmbeddingRequestOptions, EmbeddingResponse},
    error::ClientError,
    model_list::ModelListResponse,
};

use super::chat_completion::ChatCompletionStreamChunk;

pub struct Client {
    pub api_key: String,
    pub endpoint: String,
    pub max_retries: u32,
    pub timeout: u32,
}

impl Client {
    /// Constructs a new `Client`.
    ///
    /// # Arguments
    ///
    /// * `api_key`     - An optional API key.
    ///                   If not provided, the method will try to use the `MISTRAL_API_KEY` environment variable.
    /// * `endpoint`    - An optional custom API endpoint. Defaults to the official API endpoint if not provided.
    /// * `max_retries` - Optional maximum number of retries for failed requests. Defaults to `5`.
    /// * `timeout`     - Optional timeout in seconds for requests. Defaults to `120`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mistralai_client::v1::client::Client;
    ///
    /// let client = Client::new(Some("your_api_key_here".to_string()), None, Some(3), Some(60));
    /// assert!(client.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// This method fails whenever neither the `api_key` is provided
    /// nor the `MISTRAL_API_KEY` environment variable is set.
    pub fn new(
        api_key: Option<String>,
        endpoint: Option<String>,
        max_retries: Option<u32>,
        timeout: Option<u32>,
    ) -> Result<Self, ClientError> {
        let api_key = match api_key {
            Some(api_key_from_param) => api_key_from_param,
            None => std::env::var("MISTRAL_API_KEY").map_err(|_| ClientError::MissingApiKey)?,
        };
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

    /// Synchronously sends a chat completion request and returns the response.
    ///
    /// # Arguments
    ///
    /// * `model` - The [Model] to use for the chat completion.
    /// * `messages` - A vector of [ChatMessage] to send as part of the chat.
    /// * `options` - Optional [ChatCompletionParams] to customize the request.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing the `ChatCompletionResponse` if the request is successful,
    /// or an [ApiError] if there is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use mistralai_client::v1::{
    ///     chat_completion::{ChatMessage, ChatMessageRole},
    ///     client::Client,
    ///     constants::Model,
    /// };
    ///
    /// let client = Client::new(None, None, None, None).unwrap();
    /// let messages = vec![ChatMessage {
    ///     role: ChatMessageRole::user,
    ///     content: "Hello, world!".to_string(),
    /// }];
    /// let response = client.chat(Model::OpenMistral7b, messages, None).unwrap();
    /// println!("{}: {}", response.choices[0].message.role, response.choices[0].message.content);
    /// ```
    pub fn chat(
        &self,
        model: Model,
        messages: Vec<ChatMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        let request = ChatCompletionRequest::new(model, messages, false, options);

        let response = self.post_sync("/chat/completions", &request)?;
        let result = response.json::<ChatCompletionResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    /// Asynchronously sends a chat completion request and returns the response.
    ///
    /// # Arguments
    ///
    /// * `model` - The [Model] to use for the chat completion.
    /// * `messages` - A vector of [ChatMessage] to send as part of the chat.
    /// * `options` - Optional [ChatCompletionParams] to customize the request.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing a `Stream` of `ChatCompletionStreamChunk` if the request is successful,
    /// or an [ApiError] if there is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use mistralai_client::v1::{
    ///     chat_completion::{ChatMessage, ChatMessageRole},
    ///     client::Client,
    ///     constants::Model,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(None, None, None, None).unwrap();
    ///     let messages = vec![ChatMessage {
    ///         role: ChatMessageRole::user,
    ///         content: "Hello, world!".to_string(),
    ///     }];
    ///     let response = client.chat_async(Model::OpenMistral7b, messages, None).await.unwrap();
    ///     println!("{}: {}", response.choices[0].message.role, response.choices[0].message.content);
    /// }
    /// ```
    pub async fn chat_async(
        &self,
        model: Model,
        messages: Vec<ChatMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        let request = ChatCompletionRequest::new(model, messages, false, options);

        let response = self.post_async("/chat/completions", &request).await?;
        let result = response.json::<ChatCompletionResponse>().await;
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    /// Asynchronously sends a chat completion request and returns a stream of message chunks.
    ///
    /// # Arguments
    ///
    /// * `model` - The [Model] to use for the chat completion.
    /// * `messages` - A vector of [ChatMessage] to send as part of the chat.
    /// * `options` - Optional [ChatCompletionParams] to customize the request.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing a `Stream` of `ChatCompletionStreamChunk` if the request is successful,
    /// or an [ApiError] if there is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::stream::StreamExt;
    /// use mistralai_client::v1::{
    ///     chat_completion::{ChatMessage, ChatMessageRole},
    ///     client::Client,
    ///     constants::Model,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(None, None, None, None).unwrap();
    ///     let messages = vec![ChatMessage {
    ///         role: ChatMessageRole::user,
    ///         content: "Hello, world!".to_string(),
    ///     }];
    ///     let mut stream = client.chat_stream(Model::OpenMistral7b, messages, None).await.unwrap();
    ///     while let Some(chunk_result) = stream.next().await {
    ///         match chunk_result {
    ///             Ok(chunk) => {
    ///                 print!("{}", chunk.choices[0].delta.content);
    ///             }
    ///             Err(error) => {
    ///                 println!("Error: {}", error.message);
    ///             }
    ///         }
    ///     }
    /// }
    pub async fn chat_stream(
        &self,
        model: Model,
        messages: Vec<ChatMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Result<impl Stream<Item = Result<ChatCompletionStreamChunk, ApiError>>, ApiError> {
        let request = ChatCompletionRequest::new(model, messages, true, options);
        let response = self
            .post_stream("/chat/completions", &request)
            .await
            .map_err(|e| ApiError {
                message: e.to_string(),
            })?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(ApiError {
                message: format!("{}: {}", status, text),
            });
        }

        let deserialized_stream =
            response
                .bytes_stream()
                .map(|item| -> Result<ChatCompletionStreamChunk, ApiError> {
                    match item {
                        Ok(bytes) => {
                            let text = String::from_utf8(bytes.to_vec()).map_err(|e| ApiError {
                                message: e.to_string(),
                            })?;
                            let text_trimmed = text.trim_start_matches("data: ");
                            from_str(&text_trimmed).map_err(|e| ApiError {
                                message: e.to_string(),
                            })
                        }
                        Err(e) => Err(ApiError {
                            message: e.to_string(),
                        }),
                    }
                });

        Ok(deserialized_stream)
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

    pub async fn embeddings_async(
        &self,
        model: EmbedModel,
        input: Vec<String>,
        options: Option<EmbeddingRequestOptions>,
    ) -> Result<EmbeddingResponse, ApiError> {
        let request = EmbeddingRequest::new(model, input, options);

        let response = self.post_async("/embeddings", &request).await?;
        let result = response.json::<EmbeddingResponse>().await;
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

    fn build_request_stream(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let user_agent = format!(
            "ivangabriele/mistralai-client-rs/{}",
            env!("CARGO_PKG_VERSION")
        );

        let request_builder = request
            .bearer_auth(&self.api_key)
            .header("Accept", "text/event-stream")
            .header("Content-Type", "application/json")
            .header("User-Agent", user_agent);

        request_builder
    }

    fn get_sync(&self, path: &str) -> Result<reqwest::blocking::Response, ApiError> {
        let reqwest_client = reqwest::blocking::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        let request = self.build_request_sync(reqwest_client.get(url));

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

        let result = request.send().await;
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

        let result = request.send().await;
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

    async fn post_stream<T: serde::ser::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<reqwest::Response, ApiError> {
        let reqwest_client = reqwest::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        let request_builder = reqwest_client.post(url).json(params);
        let request = self.build_request_stream(request_builder);

        let result = request.send().await;
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
