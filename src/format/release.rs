use crate::format::producer::*;
use crate::format::schema::{Language, Medium, Platform};
use crate::format::vn::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct Release {
    /// Vndbid
    pub id: Option<String>,
    /// Main title as displayed on the site typically romanized from the original script
    pub title: Option<String>,
    /// Alternative title typically the same as title but in the original script
    pub alttitle: Option<String>,
    /// Languages this release is available in
    pub languages: Option<Vec<ReleaseLanguage>>,
    pub platforms: Option<Vec<Platform>>,
    pub medium: Option<Vec<ReleaseMedia>>,
    /// List of visual novels this release is linked to
    pub vns: Option<Vec<ReleaseVnRelation>>,
    pub producers: Option<Vec<ReleaseProducer>>,
    /// Release date
    pub released: Option<String>,
    /// Possibly null age rating
    pub minage: Option<u8>,
    pub patch: Option<bool>,
    pub freeware: Option<bool>,
    /// Can be null
    pub uncensored: Option<bool>,
    pub official: Option<bool>,
    pub has_ero: Option<bool>,
    /// Can be the string "non-standard" or an array of two integers indicating the width and height
    pub resolution: Option<Resolution>,
    pub engine: Option<String>,
    /// Possibly null, 1 = not voiced, 2 = only ero scenes voiced, 3 = partially voiced, 4 = fully voiced
    pub voiced: Option<VoicedType>,
    /// May contain formatting codes
    pub notes: Option<String>,
    /// Possibly null JAN/EAN/UPC code
    pub gtin: Option<String>,
    pub catalog: Option<String>,
    pub extlinks: Option<Vec<ExtLink>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleaseLanguage {
    /// Language
    pub lang: Option<Language>,
    /// Title in the original script can be null
    /// in which case the title for this language is the same as the “main” language
    pub title: Option<String>,
    /// Can be null romanized version of title
    pub latin: Option<String>,
    /// Whether this is a machine translation
    pub mtl: Option<bool>,
    /// whether this language is used to determine the “main” title for the release entry
    pub main: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleaseMedia {
    pub medium: Option<Medium>,
    /// Quantity. This is 0 for media where a quantity does not make sense, like “internet download”
    pub qty: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleaseVnRelation {
    /// The release type for this visual novel
    pub rtype: Option<ReleaseType>,
    /// Also implements all /vn fields
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
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseType {
    Trial,
    Partial,
    Complete,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ReleaseProducer {
    pub developer: Option<bool>,
    pub publisher: Option<bool>,
    /// All /producer fields are also available
    pub id: Option<String>,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub lang: Option<Language>,
    pub r#type: Option<ProducerType>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Resolution {
    NonStandard(String),
    Dimensions(Vec<u32>),
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VoicedType {
    NotVoiced = 1,
    EroScenesOnly = 2,
    PartiallyVoiced = 3,
    FullyVoiced = 4,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ExtLink {
    /// External website URL
    pub url: Option<String>,
    /// English human-readable label for this link
    pub label: Option<String>,
    /// Internal identifier of the site
    pub name: Option<String>,
    /// Remote identifier for this link
    pub id: Option<ExtLinkId>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum ExtLinkId {
    Str(String),
    Int(u32),
}
