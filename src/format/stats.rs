use serde::{Deserialize, Serialize};

/// Number of _ stored in the database
#[derive(Deserialize, Serialize, Debug)]
pub struct VndbStats {
    /// Characters
    pub chars: u32,
    pub producers: u32,
    pub releases: u32,
    pub staff: u32,
    pub tags: u32,
    pub traits: u32,
    /// Visual Novels
    pub vn: u32,
}
