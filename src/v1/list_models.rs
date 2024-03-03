use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<ListModelsModel>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListModelsModel {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<ListModelsModelPermission>,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub root: ???,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub parent: ???,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListModelsModelPermission {
    pub id: String,
    pub object: String,
    /// Unix timestamp (in seconds).
    pub created: u32,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub is_blocking: bool,
    // TODO Check this prop (seen in API responses but undocumented).
    // pub group: ???,
}
