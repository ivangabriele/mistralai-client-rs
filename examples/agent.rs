use mistralai_client::v1::{
    agent::{AgentMessage, AgentParams},
    client::Client,
};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();

    let agid = std::env::var("MISTRAL_API_AGENT")
        .expect("Please export MISTRAL_API_AGENT with the agent id you want to use");
    let messages = vec![
        AgentMessage::new_user_message("What's the best city in the world?"),
        AgentMessage::new_prefix("Valpo "),
    ];
    let options = AgentParams {
        random_seed: Some(42),
        ..Default::default()
    };

    let result = client.agent(agid, messages, Some(options)).unwrap();
    println!("Assistant: {}", result.choices[0].message.content);
    // => "Assistant: Tower. The Eiffel Tower is a famous landmark in Paris, France."
}
