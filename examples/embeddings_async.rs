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
