use serde::{Deserialize, Serialize};

/// Validates and returns information about the given API token
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthInfo {
    pub id: String,
    pub username: String,
    /// Access token holder's list read and write permissions
    pub permissions: Vec<ListPermission>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListPermission {
    ListRead,
    ListWrite,
}
