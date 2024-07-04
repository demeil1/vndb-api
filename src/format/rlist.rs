use crate::format::ulist::UListStatus;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RListPatch {
    pub status: Option<UListStatus>,
}

impl RListPatch {
    pub fn new() -> Self {
        RListPatch {
            status: Some(UListStatus::Unknown),
        }
    }

    pub fn status(mut self, s: UListStatus) -> Self {
        self.status = Some(s);
        self
    }
}
