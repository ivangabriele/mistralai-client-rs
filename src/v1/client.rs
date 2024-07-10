use futures::stream::StreamExt;
use futures::Stream;
use log::debug;
use reqwest::Error as ReqwestError;
use std::{
    any::Any,
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::v1::{chat, chat_stream, constants, embedding, error, model_list, tool, utils};

#[derive(Debug)]
pub struct Client {
    pub api_key: String,
    pub endpoint: String,
    pub max_retries: u32,
    pub timeout: u32,

    functions: Arc<Mutex<HashMap<String, Box<dyn tool::Function>>>>,
    last_function_call_result: Arc<Mutex<Option<Box<dyn Any + Send>>>>,
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
    ) -> Result<Self, error::ClientError> {
        let api_key = match api_key {
            Some(api_key_from_param) => api_key_from_param,
            None => {
                std::env::var("MISTRAL_API_KEY").map_err(|_| error::ClientError::MissingApiKey)?
            }
        };
        let endpoint = endpoint.unwrap_or(constants::API_URL_BASE.to_string());
        let max_retries = max_retries.unwrap_or(5);
        let timeout = timeout.unwrap_or(120);

        let functions: Arc<_> = Arc::new(Mutex::new(HashMap::new()));
        let last_function_call_result = Arc::new(Mutex::new(None));

        Ok(Self {
            api_key,
            endpoint,
            max_retries,
            timeout,

            functions,
            last_function_call_result,
        })
    }

    /// Synchronously sends a chat completion request and returns the response.
    ///
    /// # Arguments
    ///
    /// * `model` - The [Model] to use for the chat completion.
    /// * `messages` - A vector of [ChatMessage] to send as part of the chat.
    /// * `options` - Optional [ChatParams] to customize the request.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing the `ChatResponse` if the request is successful,
    /// or an [ApiError] if there is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use mistralai_client::v1::{
    ///     chat::{ChatMessage, ChatMessageRole},
    ///     client::Client,
    ///     constants::Model,
    /// };
    ///
    /// let client = Client::new(None, None, None, None).unwrap();
    /// let messages = vec![ChatMessage {
    ///     role: ChatMessageRole::User,
    ///     content: "Hello, world!".to_string(),
    ///     tool_calls: None,
    /// }];
    /// let response = client.chat(Model::OpenMistral7b, messages, None).unwrap();
    /// println!("{:?}: {}", response.choices[0].message.role, response.choices[0].message.content);
    /// ```
    pub fn chat(
        &self,
        model: constants::Model,
        messages: Vec<chat::ChatMessage>,
        options: Option<chat::ChatParams>,
    ) -> Result<chat::ChatResponse, error::ApiError> {
        let request = chat::ChatRequest::new(model, messages, false, options);

        let response = self.post_sync("/chat/completions", &request)?;
        let result = response.json::<chat::ChatResponse>();
        match result {
            Ok(data) => {
                utils::debug_pretty_json_from_struct("Response Data", &data);

                self.call_function_if_any(data.clone());

                Ok(data)
            }
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    /// Asynchronously sends a chat completion request and returns the response.
    ///
    /// # Arguments
    ///
    /// * `model` - The [Model] to use for the chat completion.
    /// * `messages` - A vector of [ChatMessage] to send as part of the chat.
    /// * `options` - Optional [ChatParams] to customize the request.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing a `Stream` of `ChatStreamChunk` if the request is successful,
    /// or an [ApiError] if there is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use mistralai_client::v1::{
    ///     chat::{ChatMessage, ChatMessageRole},
    ///     client::Client,
    ///     constants::Model,
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(None, None, None, None).unwrap();
    ///     let messages = vec![ChatMessage {
    ///         role: ChatMessageRole::User,
    ///         content: "Hello, world!".to_string(),
    ///         tool_calls: None,
    ///     }];
    ///     let response = client.chat_async(Model::OpenMistral7b, messages, None).await.unwrap();
    ///     println!("{:?}: {}", response.choices[0].message.role, response.choices[0].message.content);
    /// }
    /// ```
    pub async fn chat_async(
        &self,
        model: constants::Model,
        messages: Vec<chat::ChatMessage>,
        options: Option<chat::ChatParams>,
    ) -> Result<chat::ChatResponse, error::ApiError> {
        let request = chat::ChatRequest::new(model, messages, false, options);

        let response = self.post_async("/chat/completions", &request).await?;
        let result = response.json::<chat::ChatResponse>().await;
        match result {
            Ok(data) => {
                utils::debug_pretty_json_from_struct("Response Data", &data);

                self.call_function_if_any_async(data.clone()).await;

                Ok(data)
            }
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    /// Asynchronously sends a chat completion request and returns a stream of message chunks.
    ///
    /// # Arguments
    ///
    /// * `model` - The [Model] to use for the chat completion.
    /// * `messages` - A vector of [ChatMessage] to send as part of the chat.
    /// * `options` - Optional [ChatParams] to customize the request.
    ///
    /// # Returns
    ///
    /// Returns a [Result] containing a `Stream` of `ChatStreamChunk` if the request is successful,
    /// or an [ApiError] if there is an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::stream::StreamExt;
    /// use mistralai_client::v1::{
    ///     chat::{ChatMessage, ChatMessageRole},
    ///     client::Client,
    ///     constants::Model,
    /// };
    /// use std::io::{self, Write};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new(None, None, None, None).unwrap();
    ///     let messages = vec![ChatMessage {
    ///         role: ChatMessageRole::User,
    ///         content: "Hello, world!".to_string(),
    ///         tool_calls: None,
    ///     }];
    ///
    ///     let stream_result = client
    ///         .chat_stream(Model::OpenMistral7b,messages, None)
    ///         .await
    ///         .unwrap();
    ///     stream_result
    ///         .for_each(|chunk_result| async {
    ///             match chunk_result {
    ///                 Ok(chunks) => chunks.iter().for_each(|chunk| {
    ///                     print!("{}", chunk.choices[0].delta.content);
    ///                     io::stdout().flush().unwrap();
    ///                     // => "Once upon a time, [...]"
    ///                 }),
    ///                 Err(error) => {
    ///                     eprintln!("Error processing chunk: {:?}", error)
    ///                 }
    ///             }
    ///         })
    ///         .await;
    ///     print!("\n") // To persist the last chunk output.
    /// }
    pub async fn chat_stream(
        &self,
        model: constants::Model,
        messages: Vec<chat::ChatMessage>,
        options: Option<chat::ChatParams>,
    ) -> Result<
        impl Stream<Item = Result<Vec<chat_stream::ChatStreamChunk>, error::ApiError>>,
        error::ApiError,
    > {
        let request = chat::ChatRequest::new(model, messages, true, options);
        let response = self
            .post_stream("/chat/completions", &request)
            .await
            .map_err(|e| error::ApiError {
                message: e.to_string(),
            })?;
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(error::ApiError {
                message: format!("{}: {}", status, text),
            });
        }

        let deserialized_stream = response.bytes_stream().then(|bytes_result| async move {
            match bytes_result {
                Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
                    Ok(message) => {
                        let chunks = message
                            .lines()
                            .filter_map(
                                |line| match chat_stream::get_chunk_from_stream_message_line(line) {
                                    Ok(Some(chunks)) => Some(chunks),
                                    Ok(None) => None,
                                    Err(_error) => None,
                                },
                            )
                            .flatten()
                            .collect();

                        Ok(chunks)
                    }
                    Err(e) => Err(error::ApiError {
                        message: e.to_string(),
                    }),
                },
                Err(e) => Err(error::ApiError {
                    message: e.to_string(),
                }),
            }
        });

        Ok(deserialized_stream)
    }

    pub fn embeddings(
        &self,
        model: constants::EmbedModel,
        input: Vec<String>,
        options: Option<embedding::EmbeddingRequestOptions>,
    ) -> Result<embedding::EmbeddingResponse, error::ApiError> {
        let request = embedding::EmbeddingRequest::new(model, input, options);

        let response = self.post_sync("/embeddings", &request)?;
        let result = response.json::<embedding::EmbeddingResponse>();
        match result {
            Ok(data) => {
                utils::debug_pretty_json_from_struct("Response Data", &data);

                Ok(data)
            }
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub async fn embeddings_async(
        &self,
        model: constants::EmbedModel,
        input: Vec<String>,
        options: Option<embedding::EmbeddingRequestOptions>,
    ) -> Result<embedding::EmbeddingResponse, error::ApiError> {
        let request = embedding::EmbeddingRequest::new(model, input, options);

        let response = self.post_async("/embeddings", &request).await?;
        let result = response.json::<embedding::EmbeddingResponse>().await;
        match result {
            Ok(data) => {
                utils::debug_pretty_json_from_struct("Response Data", &data);

                Ok(data)
            }
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub fn get_last_function_call_result(&self) -> Option<Box<dyn Any + Send>> {
        let mut result_lock = self.last_function_call_result.lock().unwrap();

        result_lock.take()
    }

    pub fn list_models(&self) -> Result<model_list::ModelListResponse, error::ApiError> {
        let response = self.get_sync("/models")?;
        let result = response.json::<model_list::ModelListResponse>();
        match result {
            Ok(data) => {
                utils::debug_pretty_json_from_struct("Response Data", &data);

                Ok(data)
            }
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub async fn list_models_async(
        &self,
    ) -> Result<model_list::ModelListResponse, error::ApiError> {
        let response = self.get_async("/models").await?;
        let result = response.json::<model_list::ModelListResponse>().await;
        match result {
            Ok(data) => {
                utils::debug_pretty_json_from_struct("Response Data", &data);

                Ok(data)
            }
            Err(error) => Err(self.to_api_error(error)),
        }
    }

    pub fn register_function(&mut self, name: String, function: Box<dyn tool::Function>) {
        let mut functions = self.functions.lock().unwrap();

        functions.insert(name, function);
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

    fn call_function_if_any(&self, response: chat::ChatResponse) -> () {
        let next_result = match response.choices.get(0) {
            Some(first_choice) => match first_choice.message.tool_calls.to_owned() {
                Some(tool_calls) => match tool_calls.get(0) {
                    Some(first_tool_call) => {
                        let functions = self.functions.lock().unwrap();
                        match functions.get(&first_tool_call.function.name) {
                            Some(function) => {
                                let runtime = tokio::runtime::Runtime::new().unwrap();
                                let result = runtime.block_on(async {
                                    function
                                        .execute(first_tool_call.function.arguments.to_owned())
                                        .await
                                });

                                Some(result)
                            }
                            None => None,
                        }
                    }
                    None => None,
                },
                None => None,
            },
            None => None,
        };

        let mut last_result_lock = self.last_function_call_result.lock().unwrap();
        *last_result_lock = next_result;
    }

    async fn call_function_if_any_async(&self, response: chat::ChatResponse) -> () {
        let next_result = match response.choices.get(0) {
            Some(first_choice) => match first_choice.message.tool_calls.to_owned() {
                Some(tool_calls) => match tool_calls.get(0) {
                    Some(first_tool_call) => {
                        let functions = self.functions.lock().unwrap();
                        match functions.get(&first_tool_call.function.name) {
                            Some(function) => {
                                let result = function
                                    .execute(first_tool_call.function.arguments.to_owned())
                                    .await;

                                Some(result)
                            }
                            None => None,
                        }
                    }
                    None => None,
                },
                None => None,
            },
            None => None,
        };

        let mut last_result_lock = self.last_function_call_result.lock().unwrap();
        *last_result_lock = next_result;
    }

    fn get_sync(&self, path: &str) -> Result<reqwest::blocking::Response, error::ApiError> {
        let reqwest_client = reqwest::blocking::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        debug!("Request URL: {}", url);

        let request = self.build_request_sync(reqwest_client.get(url));

        let result = request.send();
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let response_status = response.status();
                    let response_body = response.text().unwrap_or_default();
                    debug!("Response Status: {}", &response_status);
                    utils::debug_pretty_json_from_string("Response Data", &response_body);

                    Err(error::ApiError {
                        message: format!("{}: {}", response_status, response_body),
                    })
                }
            }
            Err(error) => Err(error::ApiError {
                message: error.to_string(),
            }),
        }
    }

    async fn get_async(&self, path: &str) -> Result<reqwest::Response, error::ApiError> {
        let reqwest_client = reqwest::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        debug!("Request URL: {}", url);

        let request_builder = reqwest_client.get(url);
        let request = self.build_request_async(request_builder);

        let result = request.send().await;
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let response_status = response.status();
                    let response_body = response.text().await.unwrap_or_default();
                    debug!("Response Status: {}", &response_status);
                    utils::debug_pretty_json_from_string("Response Data", &response_body);

                    Err(error::ApiError {
                        message: format!("{}: {}", response_status, response_body),
                    })
                }
            }
            Err(error) => Err(error::ApiError {
                message: error.to_string(),
            }),
        }
    }

    fn post_sync<T: std::fmt::Debug + serde::ser::Serialize>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<reqwest::blocking::Response, error::ApiError> {
        let reqwest_client = reqwest::blocking::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        debug!("Request URL: {}", url);
        utils::debug_pretty_json_from_struct("Request Body", params);

        let request_builder = reqwest_client.post(url).json(params);
        let request = self.build_request_sync(request_builder);

        let result = request.send();
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let response_status = response.status();
                    let response_body = response.text().unwrap_or_default();
                    debug!("Response Status: {}", &response_status);
                    utils::debug_pretty_json_from_string("Response Data", &response_body);

                    Err(error::ApiError {
                        message: format!("{}: {}", response_body, response_status),
                    })
                }
            }
            Err(error) => Err(error::ApiError {
                message: error.to_string(),
            }),
        }
    }

    async fn post_async<T: serde::ser::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<reqwest::Response, error::ApiError> {
        let reqwest_client = reqwest::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        debug!("Request URL: {}", url);
        utils::debug_pretty_json_from_struct("Request Body", params);

        let request_builder = reqwest_client.post(url).json(params);
        let request = self.build_request_async(request_builder);

        let result = request.send().await;
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let response_status = response.status();
                    let response_body = response.text().await.unwrap_or_default();
                    debug!("Response Status: {}", &response_status);
                    utils::debug_pretty_json_from_string("Response Data", &response_body);

                    Err(error::ApiError {
                        message: format!("{}: {}", response_status, response_body),
                    })
                }
            }
            Err(error) => Err(error::ApiError {
                message: error.to_string(),
            }),
        }
    }

    async fn post_stream<T: serde::ser::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<reqwest::Response, error::ApiError> {
        let reqwest_client = reqwest::Client::new();
        let url = format!("{}{}", self.endpoint, path);
        debug!("Request URL: {}", url);
        utils::debug_pretty_json_from_struct("Request Body", params);

        let request_builder = reqwest_client.post(url).json(params);
        let request = self.build_request_stream(request_builder);

        let result = request.send().await;
        match result {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let response_status = response.status();
                    let response_body = response.text().await.unwrap_or_default();
                    debug!("Response Status: {}", &response_status);
                    utils::debug_pretty_json_from_string("Response Data", &response_body);

                    Err(error::ApiError {
                        message: format!("{}: {}", response_status, response_body),
                    })
                }
            }
            Err(error) => Err(error::ApiError {
                message: error.to_string(),
            }),
        }
    }

    fn to_api_error(&self, err: ReqwestError) -> error::ApiError {
        error::ApiError {
            message: err.to_string(),
        }
    }
}
