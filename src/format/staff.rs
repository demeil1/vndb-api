use crate::format::release::ExtLink;
use crate::format::schema::Language;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Staff {
    /// Vndbid
    pub id: Option<String>,
    /// Alias id
    pub aid: Option<u32>,
    /// Whether the ‘name’ and ‘original’ fields represent the main name for this staff entry
    pub ismain: Option<bool>,
    /// Possibly romanized name
    pub name: Option<String>,
    /// Name in original script
    pub original: Option<String>,
    /// Staff's primary language
    pub lang: Option<Language>,
    /// Male or Female
    pub gender: Option<StaffGender>,
    /// May contain formatting codes
    pub description: Option<String>,
    /// Links to external websites
    pub extlinks: Option<Vec<ExtLink>>,
    /// List of names used by this person
    pub aliases: Option<Vec<StaffAlias>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StaffGender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StaffAlias {
    /// Alias id
    pub aid: Option<u32>,
    /// Name in the original script
    pub name: Option<String>,
    /// Romanized version of ‘name’
    pub latin: Option<String>,
    /// Whether this alias is used as “main” name for the staff entry
    pub ismain: Option<bool>,
}
