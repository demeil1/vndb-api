use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    /// Array of objects representing the query results.
    pub results: Vec<T>,
    /// When true repeating the query with an incremented page number will yield more results
    pub more: bool,
    /// Indicates the total number of entries that matched the given filters
    pub count: Option<u32>,
    /// Compact string representation of the filters given in the query
    pub compact_filters: Option<String>,
    /// Normalized JSON representation of the filters given in the query
    pub normalized_filters: Option<serde_json::Value>,
}
