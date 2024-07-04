use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Trait {
    /// Vndbid
    pub id: Option<String>,
    /// Trait names are not necessarily self-describing
    /// So they should always be displayed together with their “group”
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    /// May contain formatting codes
    pub description: Option<String>,
    pub searchable: Option<bool>,
    pub applicable: Option<bool>,
    /// Vndbid
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    /// Integer number of characters this trait has been applied to including child traits
    pub char_count: Option<u32>,
}
