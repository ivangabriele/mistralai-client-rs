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

```rs
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
        tool_calls: None,
    }];
    let options = ChatParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client.chat(model, messages, Some(options)).unwrap();
    println!("Assistant: {}", result.choices[0].message.content);
    // => "Assistant: Tower. The Eiffel Tower is a famous landmark in Paris, France."
}
```

### Chat (async)

```rs
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
};

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
        tool_calls: None,
    }];
    let options = ChatParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client
        .chat_async(model, messages, Some(options))
        .await
        .unwrap();
    println!(
        "{:?}: {}",
        result.choices[0].message.role, result.choices[0].message.content
    );
    // => "Assistant: Tower. The Eiffel Tower is a famous landmark in Paris, France."
}
```

### Chat with streaming (async)

```rs
use futures::stream::StreamExt;
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
};
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: "Tell me a short happy story.".to_string(),
        tool_calls: None,
    }];
    let options = ChatParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let stream_result = client
        .chat_stream(model, messages, Some(options))
        .await
        .unwrap();
    stream_result
        .for_each(|chunk_result| async {
            match chunk_result {
                Ok(chunks) => chunks.iter().for_each(|chunk| {
                    print!("{}", chunk.choices[0].delta.content);
                    io::stdout().flush().unwrap();
                    // => "Once upon a time, [...]"
                }),
                Err(error) => {
                    eprintln!("Error processing chunk: {:?}", error)
                }
            }
        })
        .await;
    print!("\n") // To persist the last chunk output.
}
```

### Chat with Function Calling

```rs
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
    tool::{Function, Tool, ToolChoice, ToolFunctionParameter, ToolFunctionParameterType},
};
use serde::Deserialize;
use std::any::Any;

#[derive(Debug, Deserialize)]
struct GetCityTemperatureArguments {
    city: String,
}

struct GetCityTemperatureFunction;
#[async_trait::async_trait]
impl Function for GetCityTemperatureFunction {
    async fn execute(&self, arguments: String) -> Box<dyn Any + Send> {
        // Deserialize arguments, perform the logic, and return the result
        let GetCityTemperatureArguments { city } = serde_json::from_str(&arguments).unwrap();

        let temperature = match city.as_str() {
            "Paris" => "20°C",
            _ => "Unknown city",
        };

        Box::new(temperature.to_string())
    }
}

fn main() {
    let tools = vec![Tool::new(
        "get_city_temperature".to_string(),
        "Get the current temperature in a city.".to_string(),
        vec![ToolFunctionParameter::new(
            "city".to_string(),
            "The name of the city.".to_string(),
            ToolFunctionParameterType::String,
        )],
    )];

    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let mut client = Client::new(None, None, None, None).unwrap();
    client.register_function(
        "get_city_temperature".to_string(),
        Box::new(GetCityTemperatureFunction),
    );

    let model = Model::MistralSmallLatest;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: "What's the temperature in Paris?".to_string(),
        tool_calls: None,
    }];
    let options = ChatParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        tool_choice: Some(ToolChoice::Auto),
        tools: Some(tools),
        ..Default::default()
    };

    client.chat(model, messages, Some(options)).unwrap();
    let temperature = client
        .get_last_function_call_result()
        .unwrap()
        .downcast::<String>()
        .unwrap();
    println!("The temperature in Paris is: {}.", temperature);
    // => "The temperature in Paris is: 20°C."
}
```

### Chat with Function Calling (async)

```rs
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
    tool::{Function, Tool, ToolChoice, ToolFunctionParameter, ToolFunctionParameterType},
};
use serde::Deserialize;
use std::any::Any;

#[derive(Debug, Deserialize)]
struct GetCityTemperatureArguments {
    city: String,
}

struct GetCityTemperatureFunction;
#[async_trait::async_trait]
impl Function for GetCityTemperatureFunction {
    async fn execute(&self, arguments: String) -> Box<dyn Any + Send> {
        // Deserialize arguments, perform the logic, and return the result
        let GetCityTemperatureArguments { city } = serde_json::from_str(&arguments).unwrap();

        let temperature = match city.as_str() {
            "Paris" => "20°C",
            _ => "Unknown city",
        };

        Box::new(temperature.to_string())
    }
}

#[tokio::main]
async fn main() {
    let tools = vec![Tool::new(
        "get_city_temperature".to_string(),
        "Get the current temperature in a city.".to_string(),
        vec![ToolFunctionParameter::new(
            "city".to_string(),
            "The name of the city.".to_string(),
            ToolFunctionParameterType::String,
        )],
    )];

    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let mut client = Client::new(None, None, None, None).unwrap();
    client.register_function(
        "get_city_temperature".to_string(),
        Box::new(GetCityTemperatureFunction),
    );

    let model = Model::MistralSmallLatest;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: "What's the temperature in Paris?".to_string(),
        tool_calls: None,
    }];
    let options = ChatParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        tool_choice: Some(ToolChoice::Auto),
        tools: Some(tools),
        ..Default::default()
    };

    client
        .chat_async(model, messages, Some(options))
        .await
        .unwrap();
    let temperature = client
        .get_last_function_call_result()
        .unwrap()
        .downcast::<String>()
        .unwrap();
    println!("The temperature in Paris is: {}.", temperature);
    // => "The temperature in Paris is: 20°C."
}
```

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
    println!("First Embedding: {:?}", response.data[0]);
    // => "First Embedding: {...}"
}
```

### Embeddings (async)

```rs
use mistralai_client::v1::{client::Client, constants::EmbedModel};

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client: Client = Client::new(None, None, None, None).unwrap();

    let model = EmbedModel::MistralEmbed;
    let input = vec!["Embed this sentence.", "As well as this one."]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let options = None;

    let response = client
        .embeddings_async(model, input, options)
        .await
        .unwrap();
    println!("First Embedding: {:?}", response.data[0]);
    // => "First Embedding: {...}"
}
```

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

```rs
use mistralai_client::v1::client::Client;

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let result = client.list_models_async().await.unwrap();
    println!("First Model ID: {:?}", result.data[0].id);
    // => "First Model ID: open-mistral-7b"
}
```

## Contributing

Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how to contribute to this library.
