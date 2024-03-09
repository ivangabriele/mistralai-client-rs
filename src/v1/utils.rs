use std::fmt::Debug;

use log::debug;
use serde::Serialize;

pub fn prettify_json_string(json: &String) -> String {
    match serde_json::from_str::<serde_json::Value>(&json) {
        Ok(json_value) => {
            serde_json::to_string_pretty(&json_value).unwrap_or_else(|_| json.to_owned())
        }
        Err(_) => json.to_owned(),
    }
}

pub fn prettify_json_struct<T: Debug + Serialize>(value: T) -> String {
    match serde_json::to_string_pretty(&value) {
        Ok(pretty_json) => pretty_json,
        Err(_) => format!("{:?}", value),
    }
}

pub fn debug_pretty_json_from_string(label: &str, json: &String) -> () {
    let pretty_json = prettify_json_string(json);

    debug!("{label}: {}", pretty_json);
}

pub fn debug_pretty_json_from_struct<T: Debug + Serialize>(label: &str, value: &T) -> () {
    let pretty_json = prettify_json_struct(value);

    debug!("{label}: {}", pretty_json);
}
