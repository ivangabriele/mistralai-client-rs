use crate::v1::error::APIError;
use minreq::Response;

use crate::v1::{
    chat_completion::{
        ChatCompletionMessage, ChatCompletionParams, ChatCompletionRequest, ChatCompletionResponse,
    },
    constants::API_URL_BASE,
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
    ) -> Self {
        let api_key = api_key.unwrap_or(std::env::var("MISTRAL_API_KEY").unwrap());
        let endpoint = endpoint.unwrap_or(API_URL_BASE.to_string());
        let max_retries = max_retries.unwrap_or(5);
        let timeout = timeout.unwrap_or(120);

        Self {
            api_key,
            endpoint,
            max_retries,
            timeout,
        }
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

    pub fn get(&self, path: &str) -> Result<Response, APIError> {
        let url = format!("{}{}", self.endpoint, path);
        let request = self.build_request(minreq::get(url));

        let result = request.send();
        match result {
            Ok(response) => {
                if (200..=299).contains(&response.status_code) {
                    Ok(response)
                } else {
                    Err(APIError {
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
    ) -> Result<Response, APIError> {
        // print!("{:?}", params);

        let url = format!("{}{}", self.endpoint, path);
        let request = self.build_request(minreq::post(url));

        let result = request.with_json(params).unwrap().send();
        match result {
            Ok(response) => {
                // print!("{:?}", response.as_str().unwrap());

                if (200..=299).contains(&response.status_code) {
                    Ok(response)
                } else {
                    Err(APIError {
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

    pub fn delete(&self, path: &str) -> Result<Response, APIError> {
        let url = format!("{}{}", self.endpoint, path);
        let request = self.build_request(minreq::post(url));

        let result = request.send();
        match result {
            Ok(response) => {
                if (200..=299).contains(&response.status_code) {
                    Ok(response)
                } else {
                    Err(APIError {
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
        model: String,
        messages: Vec<ChatCompletionMessage>,
        options: Option<ChatCompletionParams>,
    ) -> Result<ChatCompletionResponse, APIError> {
        let request = ChatCompletionRequest::new(model, messages, options);

        let response = self.post("/chat/completions", &request)?;
        let result = response.json::<ChatCompletionResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.new_error(error)),
        }
    }

    pub fn list_models(&self) -> Result<ModelListResponse, APIError> {
        let response = self.get("/models")?;
        let result = response.json::<ModelListResponse>();
        match result {
            Ok(response) => Ok(response),
            Err(error) => Err(self.new_error(error)),
        }
    }

    fn new_error(&self, err: minreq::Error) -> APIError {
        APIError {
            message: err.to_string(),
        }
    }
}
