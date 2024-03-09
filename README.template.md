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
  - [Chat](#chat)
  - [Chat (async)](#chat-async)
  - [Chat with streaming (async)](#chat-with-streaming-async)
  - [Chat with Function Calling](#chat-with-function-calling)
  - [Chat with Function Calling (async)](#chat-with-function-calling-async)
  - [Embeddings](#embeddings)
  - [Embeddings (async)](#embeddings-async)
  - [List models](#list-models)
  - [List models (async)](#list-models-async)
- [Contributing](#contributing)

---

## Supported APIs

- [x] Chat without streaming
- [x] Chat without streaming (async)
- [x] Chat with streaming
- [x] Embedding
- [x] Embedding (async)
- [x] List models
- [x] List models (async)
- [x] Function Calling
- [x] Function Calling (async)

## Installation

You can install the library in your project using:

```sh
cargo add mistralai-client
```

### Mistral API Key

You can get your Mistral API Key there: <https://docs.mistral.ai/#api-access>.

#### As an environment variable

Just set the `MISTRAL_API_KEY` environment variable.

```rs
use mistralai_client::v1::client::Client;

fn main() {
    let client = Client::new(None, None, None, None);
}
```

```sh
MISTRAL_API_KEY=your_api_key cargo run
```

#### As a client argument

```rs
use mistralai_client::v1::client::Client;

fn main() {
    let api_key = "your_api_key";

    let client = Client::new(Some(api_key), None, None, None).unwrap();
}
```

## Usage

### Chat

<CODE>examples/chat.rs</CODE>

### Chat (async)

<CODE>examples/chat_async.rs</CODE>

### Chat with streaming (async)

<CODE>examples/chat_with_streaming.rs</CODE>

### Chat with Function Calling

<CODE>examples/chat_with_function_calling.rs</CODE>

### Chat with Function Calling (async)

<CODE>examples/chat_with_function_calling_async.rs</CODE>

### Embeddings

<CODE>examples/embeddings.rs</CODE>

### Embeddings (async)

<CODE>examples/embeddings_async.rs</CODE>

### List models

<CODE>examples/list_models.rs</CODE>

### List models (async)

<CODE>examples/list_models_async.rs</CODE>

## Contributing

Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how to contribute to this library.
