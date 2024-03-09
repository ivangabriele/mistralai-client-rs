use mistralai_client::v1::client::Client;

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let result = client.list_models_async().await.unwrap();
    println!("First Model ID: {:?}", result.data[0].id);
    // => "First Model ID: open-mistral-7b"
}
