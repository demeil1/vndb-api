use crate::format::schema::Language;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Producer {
    /// Vndbid
    pub id: Option<String>,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    /// Primary langauge
    pub lang: Option<Language>,
    pub r#type: Option<ProducerType>,
    /// May contain formatting codes
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProducerType {
    #[serde(rename = "co")]
    Company,
    #[serde(rename = "in")]
    Individual,
    #[serde(rename = "ng")]
    AmateurGroup,
}
