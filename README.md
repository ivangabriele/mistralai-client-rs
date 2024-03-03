# Mistral AI Rust Client

[![Crates.io Package](https://img.shields.io/crates/v/mistralai-client?style=for-the-badge)](https://crates.io/crates/mistralai-client)
[![Docs.rs Documentation](https://img.shields.io/docsrs/mistralai-client/latest?style=for-the-badge)](https://docs.rs/mistralai-client/latest/mistralai-client)
[![Test Workflow Status](https://img.shields.io/github/actions/workflow/status/ivangabriele/mistralai-client-rs/test.yml?label=CI&style=for-the-badge)](https://github.com/ivangabriele/mistralai-client-rs/actions?query=branch%3Amain+workflow%3ATest++)
[![Code Coverage](https://img.shields.io/codecov/c/github/ivangabriele/mistralai-client-rs/main?label=Cov&style=for-the-badge)](https://app.codecov.io/github/ivangabriele/mistralai-client-rs)

Rust client for the Mistral AI API.

---

- [Supported APIs](#supported-apis)
- [Installation](#installation)
  - [Mistral API Key](#mistral-api-key)
    - [As an environment variable](#as-an-environment-variable)
    - [As a client argument](#as-a-client-argument)
- [Usage](#usage)
  - [Chat without streaming](#chat-without-streaming)
  - [Chat with streaming](#chat-with-streaming)
  - [Embeddings](#embeddings)
  - [List models](#list-models)

---

## Supported APIs

- [x] Chat without streaming
- [ ] Chat with streaming
- [ ] Embedding
- [x] List models
- [ ] Function Calling

## Installation

You can install the library in your project using:

```sh
cargo add mistralai-client
```

### Mistral API Key

You can get your Mistral API Key there: <https://docs.mistral.ai/#api-access>.

#### As an environment variable

Just set the `MISTRAL_API_KEY` environment variable.

#### As a client argument

```rs
use mistralai_client::v1::client::Client;

fn main() {
    let api_key = "your_api_key";

    let client = Client::new(Some(api_key), None, None, None);
}
```

## Usage

### Chat without streaming

```rs
use mistralai_client::v1::{
    chat_completion::{ChatCompletionMessage, ChatCompletionMessageRole, ChatCompletionRequestOptions},
    client::Client,
    constants::OPEN_MISTRAL_7B,
};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None);

    let model = OPEN_MISTRAL_7B.to_string();
    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::user,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
    }];
    let options = ChatCompletionRequestOptions {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client.chat(model, messages, Some(options)).unwrap();
    println!("Assistant: {}", result.choices[0].message.content);
    // => "Assistant: Tower. [...]"
}
```

### Chat with streaming

_In progress._

### Embeddings

_In progress._

### List models

```rs
use mistralai_client::v1::client::Client;

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None);

    let result = client.list_models(model, messages, Some(options)).unwrap();
    println!("First Model ID: {:?}", result.data[0].id);
    // => "First Model ID: open-mistral-7b"
}
```
