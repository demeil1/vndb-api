use crate::format::producer::Producer;
use crate::format::release::{ExtLink, Release};
use crate::format::schema::{Language, Platform};
use crate::format::vn::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Character {
    pub id: Option<String>,
    pub name: Option<String>,
    /// Name in the original script
    pub original: Option<String>,
    pub aliases: Option<Vec<String>>,
    /// May contain formatting codes
    pub description: Option<String>,
    /// Same sub-fields as the image visual novel fields except for thumbnail and thumbnail_dims
    /// because character images are currently always limited to 256x300px
    /// but that is subject to change in the future
    pub image: Option<CharacterImage>,
    pub blood_type: Option<String>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub bust: Option<u32>,
    pub waist: Option<u32>,
    pub hips: Option<u32>,
    pub cup: Option<String>,
    pub age: Option<u32>,
    /// [month, day]
    pub birthday: Option<Vec<u8>>,
    /// The character’s apparent (non-spoiler) sex and the character’s real (spoiler) sex
    pub gender: Option<CharacterGenderWrapper>,
    pub vns: Option<Vec<CharacterVnRelation>>,
    pub traits: Option<Vec<CharacterTrait>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CharacterImage {
    /// includes vn image fields excluding thumbnail and thumbnail_dims
    pub id: Option<String>,
    pub url: Option<String>,
    pub dims: Option<Vec<u32>>,
    pub sexual: Option<f32>,
    pub violence: Option<f32>,
    pub votecount: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum CharacterGenderWrapper {
    Appearance(CharacterGender),
    Real(CharacterGender),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CharacterGender {
    #[serde(rename = "m")]
    Male,
    #[serde(rename = "f")]
    Female,
    #[serde(rename = "b")]
    Both,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CharacterVnRelation {
    pub spoiler: Option<u32>,
    pub role: Option<CharacterRole>,
    /// Takes all /vn fields
    pub id: Option<String>,
    pub title: Option<String>,
    pub alttitle: Option<String>,
    pub titles: Option<Vec<VnTitle>>,
    pub aliases: Option<Vec<String>>,
    pub olang: Option<Language>,
    pub devstatus: Option<VnDevStatus>,
    pub released: Option<String>,
    pub languages: Option<Vec<Language>>,
    pub platforms: Option<Vec<Platform>>,
    pub image: Option<VnImage>,
    pub length: Option<VnLength>,
    pub length_minutes: Option<u32>,
    pub length_votes: Option<u32>,
    pub description: Option<String>,
    pub rating: Option<f32>,
    pub votecount: Option<u32>,
    pub screenshots: Option<Vec<VnScreenShot>>,
    pub relations: Option<Vec<VnRelation>>,
    pub tags: Option<Vec<VnTag>>,
    pub developers: Option<Vec<Producer>>,
    pub editions: Option<Vec<VnEdition>>,
    pub staff: Option<Vec<VnStaff>>,
    pub va: Option<Vec<VnVoiceActor>>,
    pub extlinks: Option<Vec<ExtLink>>,
    pub release: Option<Release>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CharacterRole {
    Main,
    Primary,
    Side,
    Appears,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CharacterTrait {
    pub spoiler: Option<CharacterTraitSpoiler>,
    pub lie: Option<bool>,
    /// All /trait fields available here
    pub id: Option<String>,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    pub searchable: Option<bool>,
    pub applicable: Option<bool>,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub char_count: Option<u32>,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum CharacterTraitSpoiler {
    None = 0,
    Medium = 1,
    Big = 2,
}
