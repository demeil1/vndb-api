use serde::{Deserialize, Serialize};
use serde_repr::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::format::release::*;
use crate::format::schema::Platform;
use crate::format::vn::VisualNovel;

#[derive(Deserialize, Serialize, Debug)]
pub struct UList {
    /// Vn id
    pub id: Option<String>,
    /// Unix timestamp
    pub added: Option<u64>,
    /// Unix timestamp of when the user voted on this VN
    pub voted: Option<u64>,
    /// Unix timestamp when the user last modified their list for this VN
    pub lastmod: Option<u64>,
    /// 10 - 100
    pub vote: Option<u8>,
    /// Start date “YYYY-MM-DD” format
    pub started: Option<String>,
    /// Finish date
    pub finished: Option<String>,
    pub notes: Option<String>,
    /// User labels assigned to this VN private labels are only listed when the user is authenticated
    pub labels: Option<Vec<UListLabel>>,
    pub vn: Option<VisualNovel>,
    pub releases: Option<Vec<UListRelease>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UListLabel {
    pub id: Option<u32>,
    pub label: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UListRelease {
    pub list_status: Option<UListStatus>,
    /// All /release fields can be selected here
    pub id: Option<String>,
    pub title: Option<String>,
    pub alttitle: Option<String>,
    pub languages: Option<Vec<ReleaseLanguage>>,
    pub platforms: Option<Vec<Platform>>,
    pub medium: Option<Vec<ReleaseMedia>>,
    pub vns: Option<Vec<ReleaseVnRelation>>,
    pub producers: Option<Vec<ReleaseProducer>>,
    pub released: Option<String>,
    pub minage: Option<u8>,
    pub patch: Option<bool>,
    pub freeware: Option<bool>,
    pub uncensored: Option<bool>,
    pub official: Option<bool>,
    pub has_ero: Option<bool>,
    pub resolution: Option<Resolution>,
    pub engine: Option<String>,
    pub voiced: Option<VoicedType>,
    pub notes: Option<String>,
    pub gtin: Option<String>,
    pub catalog: Option<String>,
    pub extlinks: Option<Vec<ExtLink>>,
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum UListStatus {
    Unknown = 0,
    Pending = 1,
    Obtained = 2,
    OnLoan = 3,
    Deleted = 4,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UListLabels {
    pub labels: Option<Vec<UListLabelsInst>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UListLabelsInst {
    /// Integer identifier of the label
    pub id: Option<u32>,
    /// Whether this label is private, private labels are only included when authenticated with the listread permission
    pub private: Option<bool>,
    pub label: Option<String>,
    pub count: Option<u32>,
}

pub struct UListLabelsFieldChoices(pub Vec<UListLabelsField>);

#[derive(Serialize, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum UListLabelsField {
    Count,
}

impl UListLabelsFieldChoices {
    pub fn new() -> Self {
        UListLabelsFieldChoices(vec![])
    }

    pub fn from(vec: Vec<UListLabelsField>) -> Self {
        UListLabelsFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(UListLabelsField::iter().len());
        for variant in UListLabelsField::iter() {
            vec.push(variant);
        }
        UListLabelsFieldChoices(vec)
    }

    pub fn to_csv(&self) -> String {
        self.0
            .iter()
            .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UListPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    vote: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    started: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    finished: Option<String>,
    /// Setting this will overwrite any existing labels assigned to the VN with the given array
    #[serde(skip_serializing_if = "Option::is_none")]
    labels: Option<Vec<LabelId>>,
    /// Label ids to add to the VN any already existing labels will be unaffected
    #[serde(skip_serializing_if = "Option::is_none")]
    labels_set: Option<Vec<LabelId>>,
    /// Label ids to remove from the VN
    #[serde(skip_serializing_if = "Option::is_none")]
    labels_unset: Option<Vec<LabelId>>,
}

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    fn format(self) -> String {
        let s = format!("{:04}-{:02}-{:02}", self.year, self.month, self.day);
        s
    }
}

#[derive(Deserialize_repr, Serialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum LabelId {
    Playing = 1,
    Finished = 2,
    Stalled = 3,
    Dropped = 4,
    WishList = 5,
    BlackList = 6,
}

pub struct UListPatchBuilder {
    pub vote: Option<u8>,
    pub notes: Option<String>,
    pub started: Option<String>,
    pub finished: Option<String>,
    pub labels: Option<Vec<LabelId>>,
    pub labels_set: Option<Vec<LabelId>>,
    pub labels_unset: Option<Vec<LabelId>>,
}

impl UListPatchBuilder {
    pub fn new() -> Self {
        UListPatchBuilder {
            vote: None,
            notes: None,
            started: None,
            finished: None,
            labels: None,
            labels_set: None,
            labels_unset: None,
        }
    }

    pub fn vote(mut self, num: u8) -> Self {
        self.vote = Some(num.clamp(10, 100));
        self
    }

    pub fn notes(mut self, ctx: String) -> Self {
        self.notes = Some(ctx);
        self
    }

    pub fn started(mut self, start: Date) -> Self {
        self.started = Some(start.format());
        self
    }

    pub fn finished(mut self, finish: Date) -> Self {
        self.finished = Some(finish.format());
        self
    }

    pub fn labels(mut self, l: Vec<LabelId>) -> Self {
        self.labels = Some(l);
        self
    }

    pub fn labels_set(mut self, l: Vec<LabelId>) -> Self {
        self.labels_set = Some(l);
        self
    }

    pub fn labels_unset(mut self, l: Vec<LabelId>) -> Self {
        self.labels_unset = Some(l);
        self
    }

    pub fn build(self) -> UListPatch {
        UListPatch {
            vote: self.vote,
            notes: self.notes,
            started: self.started,
            finished: self.finished,
            labels: self.labels,
            labels_set: self.labels_set,
            labels_unset: self.labels_unset,
        }
    }
}
