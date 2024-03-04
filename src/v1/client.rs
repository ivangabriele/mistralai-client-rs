use crate::v1::error::ApiError;
use minreq::Response;

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

    pub fn build_request(&self, request: minreq::Request) -> minreq::Request {
        let authorization = format!("Bearer {}", self.api_key);
        let user_agent = format!(
            "ivangabriele/mistralai-client-rs/{}",
            env!("CARGO_PKG_VERSION")
        );

        let request = request
            .with_header("Authorization", authorization)
            .with_header("Accept", "application/json")
            .with_header("Content-Type", "application/json")
            .with_header("User-Agent", user_agent);

        request
    }

    pub fn get(&self, path: &str) -> Result<Response, ApiError> {
        let url = format!("{}{}", self.endpoint, path);
        let request = self.build_request(minreq::get(url));

        let result = request.send();
        match result {
            Ok(response) => {
                print!("{:?}", response.as_str().unwrap());

                if (200..=299).contains(&response.status_code) {
                    Ok(response)
                } else {
                    Err(ApiError {
                        message: format!(
                            "{}: {}",
                            response.status_code,
                            response.as_str().unwrap()
                        ),
                    })
                }
            }
            Err(error) => Err(self.new_error(error)),
        }
    }

    pub fn post<T: serde::ser::Serialize + std::fmt::Debug>(
        &self,
        path: &str,
        params: &T,
    ) -> Result<Response, ApiError> {
        // print!("{:?}", params);

        let url = format!("{}{}", self.endpoint, path);
        let request = self.build_request(minreq::post(url));

        let result = request.with_json(params).unwrap().send();
        match result {
            Ok(response) => {
                print!("{:?}", response.as_str().unwrap());

                if (200..=299).contains(&response.status_code) {
                    Ok(response)
                } else {
                    Err(ApiError {
                        message: format!(
                            "{}: {}",
                            response.status_code,
                            response.as_str().unwrap()
                        ),
                    })
                }
            }
            Err(error) => Err(self.new_error(error)),
        }
    }

    pub fn chat(
        &self,
        model: Model,
        messages: Vec<ChatCompletionMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        let request = ChatCompletionRequest::new(model, messages, options);

        let response = self.post("/chat/completions", &request)?;
        let result = response.json::<ChatCompletionResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.new_error(error)),
        }
    }

    pub fn embeddings(
        &self,
        model: EmbedModel,
        input: Vec<String>,
        options: Option<EmbeddingRequestOptions>,
    ) -> Result<EmbeddingResponse, ApiError> {
        let request = EmbeddingRequest::new(model, input, options);

        let response = self.post("/embeddings", &request)?;
        let result = response.json::<EmbeddingResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.new_error(error)),
        }
    }

    pub fn list_models(&self) -> Result<ModelListResponse, ApiError> {
        let response = self.get("/models")?;
        let result = response.json::<ModelListResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.new_error(error)),
        }
    }

    fn new_error(&self, err: minreq::Error) -> ApiError {
        ApiError {
            message: err.to_string(),
        }
    }
}
