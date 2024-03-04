use jrest::expect;
use mistralai_client::v1::{client::Client, constants::EmbedModel};

#[test]
fn test_client_embeddings() {
    let client: Client = Client::new(None, None, None, None);

    let model = EmbedModel::MistralEmbed;
    let input = vec!["Embed this sentence.", "As well as this one."]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let options = None;

    let response = client.embeddings(model, input, options).unwrap();

    expect!(response.model).to_be(EmbedModel::MistralEmbed);
    expect!(response.object).to_be("list".to_string());
    expect!(response.data.len()).to_be(2);
    expect!(response.data[0].index).to_be(0);
    expect!(response.data[0].object.clone()).to_be("embedding".to_string());
    expect!(response.data[0].embedding.len()).to_be_greater_than(0);
    expect!(response.usage.prompt_tokens).to_be_greater_than(0);
    expect!(response.usage.completion_tokens).to_be(0);
    expect!(response.usage.total_tokens).to_be_greater_than(0);
}
