use crate::format::character::*;
use crate::format::producer::Producer;
use crate::format::release::*;
use crate::format::schema::{Language, Platform, StaffRole};
use crate::format::staff::*;
use crate::format::tag::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct VisualNovel {
    /// Vndbid
    pub id: Option<String>,
    pub title: Option<String>,
    pub alttitle: Option<String>,
    /// Full list of titles associated with the VN
    pub titles: Option<Vec<VnTitle>>,
    pub aliases: Option<Vec<String>>,
    /// Language the VN has originally been written in
    pub olang: Option<Language>,
    pub devstatus: Option<VnDevStatus>,
    /// Release date
    pub released: Option<String>,
    /// List of languages this VN is available in does not include machine translations
    pub languages: Option<Vec<Language>>,
    /// List of platforms for which this VN is available
    pub platforms: Option<Vec<Platform>>,
    pub image: Option<VnImage>,
    /// Rough length estimate of the VN between very short and very long
    pub length: Option<VnLength>,
    /// Average of user-submitted play times in minutes
    pub length_minutes: Option<u32>,
    /// Number of submitted play times
    pub length_votes: Option<u32>,
    /// May contain formatting codes
    pub description: Option<String>,
    /// The rating of a VN from 10 to 100
    pub rating: Option<f32>,
    /// Number of votes
    pub votecount: Option<u32>,
    pub screenshots: Option<Vec<VnScreenShot>>,
    /// List of VNs directly related to this entry
    pub relations: Option<Vec<VnRelation>>,
    /// VN tags
    pub tags: Option<Vec<VnTag>>,
    /// All producers with a “developer” role on a release linked to the VN
    pub developers: Option<Vec<Producer>>,
    /// List of VN editions
    pub editions: Option<Vec<VnEdition>>,
    /// List of staff members
    pub staff: Option<Vec<VnStaff>>,
    /// List of voice actors the same voice actor may be listed multiple times
    /// for different characters and the same character may be listed multiple times
    /// if it has been voiced by several people
    pub va: Option<Vec<VnVoiceActor>>,
    /// Links to external websites
    pub extlinks: Option<Vec<ExtLink>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VnTitle {
    /// Language
    pub lang: Option<Language>,
    /// Title in the original script
    pub title: Option<String>,
    /// Romanized version of title
    pub latin: Option<String>,
    /// Whether this title is the official title of the VN
    pub official: Option<bool>,
    /// Whether this is the "main" title for the VN entry
    pub main: Option<bool>,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VnDevStatus {
    Finished = 0,
    InDevelopment = 1,
    Cancelled = 2,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VnImage {
    pub id: Option<String>,
    pub url: Option<String>,
    /// Pixel dimensions of the image [width, height]
    pub dims: Option<Vec<u32>>,
    /// Average image flagging vote for sexual content between 0 and 2 (inclusive)
    pub sexual: Option<f32>,
    /// Average image flagging vote for violence between 0 and 2 (inclusive)
    pub violence: Option<f32>,
    /// Number of image flagging votes
    pub votecount: Option<u32>,
    /// URL to the thumbnail
    pub thumbnail: Option<String>,
    /// Pixel dimensions of the thumbnail [width, height]
    pub thumbnail_dims: Option<Vec<u32>>,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum VnLength {
    VeryShort = 1,
    Short = 2,
    Average = 3,
    Long = 4,
    VeryLong = 5,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VnScreenShot {
    /// Image fields also apply to screenshots
    pub id: Option<String>,
    pub url: Option<String>,
    pub dims: Option<Vec<u32>>,
    pub sexual: Option<f32>,
    pub violence: Option<f32>,
    pub votecount: Option<u32>,
    pub thumbnail: Option<String>,
    pub thumbnail_dims: Option<Vec<u32>>,
    // Includes a release object
    pub release: Option<Release>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VnRelation {
    /// Relation type
    pub relation: Option<String>,
    pub relation_official: Option<bool>,
    /// all visual novel fields need to be selected here
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
pub struct VnTag {
    /// Tag rating between 0 (exclusive) and 3 (inclusive)
    pub rating: Option<f32>,
    /// Spoiler level
    pub spoiler: Option<TagSpoilerLvl>,
    pub lie: Option<bool>,
    /// Also takes all /tag fields
    pub id: Option<String>,
    pub name: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub description: Option<String>,
    pub category: Option<TagFlag>,
    pub searchable: Option<bool>,
    pub vn_count: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VnEdition {
    /// Edition identifier local to the visual novel
    /// Not stable across edits of the VN entry
    /// Only used for organizing the staff listing
    pub eid: Option<u16>,
    pub lang: Option<Language>,
    /// English name / label identifying this edition
    pub name: Option<String>,
    /// Whether this edition is official
    pub official: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VnStaff {
    /// Edition identifier when the staff has worked on the “original” version of the VN
    pub eid: Option<u32>,
    /// Staff member's role
    pub role: Option<StaffRole>,
    pub note: Option<String>,
    /// Also implements all /staff fields
    pub id: Option<String>,
    pub aid: Option<u32>,
    pub ismain: Option<bool>,
    pub name: Option<String>,
    pub original: Option<String>,
    pub lang: Option<Language>,
    pub gender: Option<StaffGender>,
    pub description: Option<String>,
    pub extlinks: Option<Vec<ExtLink>>,
    pub aliases: Option<Vec<StaffAlias>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VnVoiceActor {
    pub note: Option<String>,
    pub staff: Option<Staff>,
    pub character: Option<Character>,
}
