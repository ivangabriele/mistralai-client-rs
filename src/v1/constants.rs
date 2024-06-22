use serde::{Deserialize, Serialize};

pub const API_URL_BASE: &str = "https://api.mistral.ai/v1";

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum Model {
    #[serde(rename = "open-mistral-7b")]
    OpenMistral7b,
    #[serde(rename = "open-mixtral-8x7b")]
    OpenMixtral8x7b,
    #[serde(rename = "open-mixtral-8x22b")]
    OpenMixtral8x22b,
    #[serde(rename = "mistral-tiny")]
    MistralTiny,
    #[serde(rename = "mistral-small-latest")]
    MistralSmallLatest,
    #[serde(rename = "mistral-medium-latest")]
    MistralMediumLatest,
    #[serde(rename = "mistral-large-latest")]
    MistralLargeLatest,
    #[serde(rename = "codestral-latest")]
    CodestralLatest,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum EmbedModel {
    #[serde(rename = "mistral-embed")]
    MistralEmbed,
}
