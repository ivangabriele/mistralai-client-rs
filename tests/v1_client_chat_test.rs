use jrest::expect;
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams, ChatResponseChoiceFinishReason},
    client::Client,
    constants::Model,
    tool::{Tool, ToolChoice, ToolFunctionParameter, ToolFunctionParameterType},
};

mod setup;

#[test]
fn test_client_chat() {
    setup::setup();

    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::OpenMistral7b;
    let messages = vec![ChatMessage::new_user_message(
        "Just guess the next word: \"Eiffel ...\"?",
    )];
    let options = ChatParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        ..Default::default()
    };

    let response = client.chat(model, messages, Some(options)).unwrap();

    expect!(response.model).to_be(Model::OpenMistral7b);
    expect!(response.object).to_be("chat.completion".to_string());
    expect!(response.choices.len()).to_be(1);
    expect!(response.choices[0].index).to_be(0);
    expect!(response.choices[0].message.role.clone()).to_be(ChatMessageRole::Assistant);
    expect!(response.choices[0].message.content.clone())
        .to_be("Tower. The Eiffel Tower is a famous landmark in Paris, France.".to_string());
    expect!(response.choices[0].finish_reason.clone()).to_be(ChatResponseChoiceFinishReason::Stop);
    expect!(response.usage.prompt_tokens).to_be_greater_than(0);
    expect!(response.usage.completion_tokens).to_be_greater_than(0);
    expect!(response.usage.total_tokens).to_be_greater_than(0);
}

#[test]
fn test_client_chat_with_function_calling() {
    setup::setup();

    let tools = vec![Tool::new(
        "get_city_temperature".to_string(),
        "Get the current temperature in a city.".to_string(),
        vec![ToolFunctionParameter::new(
            "city".to_string(),
            "The name of the city.".to_string(),
            ToolFunctionParameterType::String,
        )],
    )];

    let client = Client::new(None, None, None, None).unwrap();

    let model = Model::MistralSmallLatest;
    let messages = vec![ChatMessage::new_user_message(
        "What's the current temperature in Paris?",
    )];
    let options = ChatParams {
        temperature: Some(0.0),
        random_seed: Some(42),
        tool_choice: Some(ToolChoice::Auto),
        tools: Some(tools),
        ..Default::default()
    };

    let response = client.chat(model, messages, Some(options)).unwrap();

    expect!(response.model).to_be(Model::MistralSmallLatest);
    expect!(response.object).to_be("chat.completion".to_string());
    expect!(response.choices.len()).to_be(1);
    expect!(response.choices[0].index).to_be(0);
    expect!(response.choices[0].message.role.clone()).to_be(ChatMessageRole::Assistant);
    expect!(response.choices[0].message.content.clone()).to_be("".to_string());
    expect!(response.choices[0].finish_reason.clone())
        .to_be(ChatResponseChoiceFinishReason::ToolCalls);
    expect!(response.usage.prompt_tokens).to_be_greater_than(0);
    expect!(response.usage.completion_tokens).to_be_greater_than(0);
    expect!(response.usage.total_tokens).to_be_greater_than(0);
}
