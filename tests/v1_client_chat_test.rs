use jrest::expect;
use mistralai_client::v1::{
    chat_completion::{ChatCompletionMessage, ChatCompletionMessageRole, ChatCompletionParams},
    client::Client,
    constants::Model,
};

#[test]
fn test_client_chat() {
    let client = Client::new(None, None, None, None);

    let model = Model::OpenMistral7b;
    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::user,
        content: "Just guess the next word: \"Eiffel ...\"?".to_string(),
    }];
    let options = ChatCompletionParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let response = client.chat(model, messages, Some(options)).unwrap();

    expect!(response.model).to_be(Model::OpenMistral7b);
    expect!(response.object).to_be("chat.completion".to_string());
    expect!(response.choices.len()).to_be(1);
    expect!(response.choices[0].index).to_be(0);
    expect!(response.choices[0].message.role.clone()).to_be(ChatCompletionMessageRole::assistant);
    expect!(response.choices[0].message.content.clone())
        .to_be("Tower. The Eiffel Tower is a famous landmark in Paris, France.".to_string());
    expect!(response.usage.prompt_tokens).to_be_greater_than(0);
    expect!(response.usage.completion_tokens).to_be_greater_than(0);
    expect!(response.usage.total_tokens).to_be_greater_than(0);
}
