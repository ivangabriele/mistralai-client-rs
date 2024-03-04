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
  - [Chat without streaming (async)](#chat-without-streaming-async)
  - [Chat with streaming](#chat-with-streaming)
  - [Embeddings](#embeddings)
  - [Embeddings (async)](#embeddings-async)
  - [List models](#list-models)
  - [List models (async)](#list-models-async)

---

## Supported APIs

- [x] Chat without streaming
- [x] Chat without streaming (async)
- [ ] Chat with streaming
- [x] Embedding
- [ ] Embedding (async)
- [x] List models
- [ ] List models (async)
- [ ] Function Calling
- [ ] Function Calling (async)

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

    let client = Client::new(Some(api_key), None, None, None).unwrap();
}
```

## Usage

### Chat without streaming

```rs
use mistralai_client::v1::{
    chat_completion::{ChatCompletionMessage, ChatCompletionMessageRole, ChatCompletionRequestOptions},
    client::Client,
    constants::Model,
};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
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

### Chat without streaming (async)

```rs
use mistralai_client::v1::{
    chat_completion::{ChatCompletionMessage, ChatCompletionMessageRole, ChatCompletionRequestOptions},
    client::Client,
    constants::Model,
};

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::user,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
    }];
    let options = ChatCompletionRequestOptions {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client.chat(model, messages, Some(options)).await.unwrap();
    println!("Assistant: {}", result.choices[0].message.content);
    // => "Assistant: Tower. [...]"
}
```

### Chat with streaming

_In progress._

### Embeddings

```rs
use mistralai_client::v1::{client::Client, constants::EmbedModel};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
  let client: Client = Client::new(None, None, None, None).unwrap();

  let model = EmbedModel::MistralEmbed;
  let input = vec!["Embed this sentence.", "As well as this one."]
      .iter()
      .map(|s| s.to_string())
      .collect();
  let options = None;

  let response = client.embeddings(model, input, options).unwrap();
  println!("Embeddings: {:?}", response.data);
  // => "Embeddings: [{...}, {...}]"
}
```

### Embeddings (async)

_In progress._

### List models

```rs
use mistralai_client::v1::client::Client;

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let result = client.list_models().unwrap();
    println!("First Model ID: {:?}", result.data[0].id);
    // => "First Model ID: open-mistral-7b"
}
```

### List models (async)

_In progress._
