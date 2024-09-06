use mistralai_client::v1::{
    agent::{AgentMessage, AgentParams},
    client::Client,
    tool::{Function, Tool, ToolChoice, ToolFunctionParameter, ToolFunctionParameterType},
};
use serde::Deserialize;
use std::any::Any;

#[derive(Debug, Deserialize)]
struct GetCityTemperatureArguments {
    city: String,
}

struct GetCityTemperatureFunction;
#[async_trait::async_trait]
impl Function for GetCityTemperatureFunction {
    async fn execute(&self, arguments: String) -> Box<dyn Any + Send> {
        // Deserialize arguments, perform the logic, and return the result
        let GetCityTemperatureArguments { city } = serde_json::from_str(&arguments).unwrap();

        let temperature = match city.as_str() {
            "Paris" => "20°C",
            _ => "Unknown city",
        };

        Box::new(temperature.to_string())
    }
}

fn main() {
    let tools = vec![Tool::new(
        "get_city_temperature".to_string(),
        "Get the current temperature in a city.".to_string(),
        vec![ToolFunctionParameter::new(
            "city".to_string(),
            "The name of the city.".to_string(),
            ToolFunctionParameterType::String,
        )],
    )];

    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let mut client = Client::new(None, None, None, None).unwrap();
    client.register_function(
        "get_city_temperature".to_string(),
        Box::new(GetCityTemperatureFunction),
    );

    let agid = std::env::var("MISTRAL_API_AGENT")
        .expect("Please export MISTRAL_API_AGENT with the agent id you want to use");
    let messages = vec![AgentMessage::new_user_message(
        "What's the temperature in Paris?",
    )];
    let options = AgentParams {
        random_seed: Some(42),
        tool_choice: Some(ToolChoice::Auto),
        tools: Some(tools),
        ..Default::default()
    };

    client.agent(agid, messages, Some(options)).unwrap();
    let temperature = client
        .get_last_function_call_result()
        .unwrap()
        .downcast::<String>()
        .unwrap();
    println!("The temperature in Paris is: {}.", temperature);
    // => "The temperature in Paris is: 20°C."
}
