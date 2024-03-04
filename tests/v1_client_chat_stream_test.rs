use futures::stream::StreamExt;
use jrest::expect;
use mistralai_client::v1::{
    chat_completion::{ChatCompletionParams, ChatMessage, ChatMessageRole},
    client::Client,
    constants::Model,
};

#[tokio::test]
async fn test_client_chat_stream() {
    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage {
        role: ChatMessageRole::user,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
    }];
    let options = ChatCompletionParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let stream_result = client.chat_stream(model, messages, Some(options)).await;
    let mut stream = stream_result.expect("Failed to create stream.");
    while let Some(chunk_result) = stream.next().await {
        match chunk_result {
            Ok(chunk) => {
                if chunk.choices[0].delta.role == Some(ChatMessageRole::assistant)
                    || chunk.choices[0].finish_reason == Some("stop".to_string())
                {
                    expect!(chunk.choices[0].delta.content.len()).to_be(0);
                } else {
                    expect!(chunk.choices[0].delta.content.len()).to_be_greater_than(0);
                }
            }
            Err(e) => eprintln!("Error processing chunk: {:?}", e),
        }
    }
}
