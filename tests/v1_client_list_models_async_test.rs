use jrest::expect;
use mistralai_client::v1::client::Client;

#[tokio::test]
async fn test_client_list_models_async() {
    let client = Client::new(None, None, None, None).unwrap();

    let response = client.list_models_async().await.unwrap();

    expect!(response.object).to_be("list".to_string());
    expect!(response.data.len()).to_be_greater_than(0);
}
