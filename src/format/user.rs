use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// Contains results of a user search
#[derive(Deserialize, Serialize, Debug)]
pub struct UserSearch(pub HashMap<String, Option<User>>);

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    /// Searched user's number of play time votes submitted
    pub lengthvotes: Option<u32>,
    /// Searched user's play time votes sum in minutes
    pub lengthvotes_sum: Option<u32>,
}

/// Contains the desired data fields for a user search
pub struct UserSearchFields(pub Vec<UserDataField>);

#[derive(Serialize, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum UserDataField {
    /// The number of play time votes this user has submitted
    Lengthvotes,
    /// The sum of the user’s play time votes in minutes
    LengthvotesSum,
}

impl UserSearchFields {
    /// Include the id and username only
    pub fn new() -> Self {
        UserSearchFields(vec![])
    }

    /// Include the desired UserDataFields in addition to id and username
    pub fn from(vec: Vec<UserDataField>) -> Self {
        UserSearchFields(vec)
    }

    /// Include all UserDataFields in addition to id and username
    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(UserDataField::iter().len());
        for variant in UserDataField::iter() {
            vec.push(variant);
        }
        UserSearchFields(vec)
    }

    /// Format UserSearchFields for user queries
    pub fn to_csv(&self) -> String {
        self.0
            .iter()
            .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
            .collect::<Vec<String>>()
            .join(",")
    }
}
