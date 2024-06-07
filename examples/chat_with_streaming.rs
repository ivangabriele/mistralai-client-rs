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
        temperature: 0.0,
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
