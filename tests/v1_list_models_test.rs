use jrest::expect;
use mistralai_client::v1::client::Client;

#[test]
fn test_list_models() {
    extern crate dotenv;

    use dotenv::dotenv;
    dotenv().ok();

    let client = Client::new(None, None, None, None);

    let response = client.list_models().unwrap();

    expect!(response.object).to_be("list".to_string());
    expect!(response.data.len()).to_be_greater_than(0);

    // let open_mistral_7b_data_item = response
    //     .data
    //     .iter()
    //     .find(|item| item.id == "open-mistral-7b")
    //     .unwrap();

    // expect!(open_mistral_7b_data_item.id).to_be("open-mistral-7b".to_string());
}
