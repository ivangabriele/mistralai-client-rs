use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiError: {}", self.message)
    }
}
impl Error for ApiError {}

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("You must either set the `MISTRAL_API_KEY` environment variable or specify it in `Client::new(api_key, ...).")]
    ApiKeyError,
}
