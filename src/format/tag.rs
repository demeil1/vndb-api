use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Tag {
    /// Vndbid
    pub id: Option<String>,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    /// May contain formatting codes
    pub description: Option<String>,
    pub category: Option<TagFlag>,
    pub searchable: Option<bool>,
    /// Number of VNs this tag has been applied to including any child tag
    pub vn_count: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum TagFlag {
    #[serde(rename = "cont")]
    Content,
    #[serde(rename = "ero")]
    SexualContent,
    #[serde(rename = "tech")]
    Technical,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum TagSpoilerLvl {
    None = 0,
    Medium = 1,
    Big = 2,
}
