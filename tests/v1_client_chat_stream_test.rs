// use futures::stream::StreamExt;
// use jrest::expect;
// use mistralai_client::v1::{
//     chat_completion::{ChatParams, ChatMessage, ChatMessageRole},
//     client::Client,
//     constants::Model,
// };

// #[tokio::test]
// async fn test_client_chat_stream() {
//     let client = Client::new(None, None, None, None).unwrap();

//     let model = Model::OpenMistral7b;
//     let messages = vec![ChatMessage::new_user_message(
//         "Just guess the next word: \"Eiffel ...\"?",
//     )];
//     let options = ChatParams {
//         temperature: Some(0.0),
//         random_seed: Some(42),
//         ..Default::default()
//     };

//     let stream_result = client.chat_stream(model, messages, Some(options)).await;
//     let mut stream = stream_result.expect("Failed to create stream.");
//     while let Some(maybe_chunk_result) = stream.next().await {
//         match maybe_chunk_result {
//             Some(Ok(chunk)) => {
//                 if chunk.choices[0].delta.role == Some(ChatMessageRole::Assistant)
//                     || chunk.choices[0].finish_reason == Some("stop".to_string())
//                 {
//                     expect!(chunk.choices[0].delta.content.len()).to_be(0);
//                 } else {
//                     expect!(chunk.choices[0].delta.content.len()).to_be_greater_than(0);
//                 }
//             }
//             Some(Err(error)) => eprintln!("Error processing chunk: {:?}", error),
//             None => (),
//         }
//     }
// }
