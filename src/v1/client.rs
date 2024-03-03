use crate::v1::error::APIError;
use minreq::Response;

use crate::v1::{
    chat_completion::{ChatCompletionRequest, ChatCompletionResponse},
    constants::API_URL_BASE,
};

use super::chat_completion::{ChatCompletionMessage, ChatCompletionParams};

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
            "ivangabriele/mistral-client-rs/{}",
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
        let request = self.build_request(minreq::post(url));

        let result = request.send();
        match result {
            Ok(res) => {
                if (200..=299).contains(&res.status_code) {
                    Ok(res)
                } else {
                    Err(APIError {
                        message: format!("{}: {}", res.status_code, res.as_str().unwrap()),
                    })
                }
            }
            Err(e) => Err(self.new_error(e)),
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
                print!("{:?}", response.as_str().unwrap());

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

    // pub fn completion(&self, req: CompletionRequest) -> Result<CompletionResponse, APIError> {
    //     let res = self.post("/completions", &req)?;
    //     let r = res.json::<CompletionResponse>();
    //     match r {
    //         Ok(r) => Ok(r),
    //         Err(e) => Err(self.new_error(e)),
    //     }
    // }

    // pub fn embedding(&self, req: EmbeddingRequest) -> Result<EmbeddingResponse, APIError> {
    //     let res = self.post("/embeddings", &req)?;
    //     let r = res.json::<EmbeddingResponse>();
    //     match r {
    //         Ok(r) => Ok(r),
    //         Err(e) => Err(self.new_error(e)),
    //     }
    // }

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

    fn new_error(&self, err: minreq::Error) -> APIError {
        APIError {
            message: err.to_string(),
        }
    }

    // fn query_params(
    //     limit: Option<i64>,
    //     order: Option<String>,
    //     after: Option<String>,
    //     before: Option<String>,
    //     mut url: String,
    // ) -> String {
    //     let mut params = vec![];
    //     if let Some(limit) = limit {
    //         params.push(format!("limit={}", limit));
    //     }
    //     if let Some(order) = order {
    //         params.push(format!("order={}", order));
    //     }
    //     if let Some(after) = after {
    //         params.push(format!("after={}", after));
    //     }
    //     if let Some(before) = before {
    //         params.push(format!("before={}", before));
    //     }
    //     if !params.is_empty() {
    //         url = format!("{}?{}", url, params.join("&"));
    //     }
    //     url
    // }
}
