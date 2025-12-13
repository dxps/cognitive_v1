use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::domain::model::Id;

/// User account contains most of the details of a user (except password related ones).
#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct UserAccount {
    pub id: Id,
    pub email: String,
    pub username: String,
    pub bio: String,
    pub is_anonymous: bool,
    pub permissions: Vec<String>,
}

#[cfg(feature = "server")]
impl Default for UserAccount {
    fn default() -> Self {
        Self {
            id: Id::new(),
            is_anonymous: true,
            username: "Guest".into(),
            email: "".into(),
            bio: "".into(),
            permissions: Vec::new(),
        }
    }
}

#[derive(Debug)]
/// It includes all user attributes that are persisted in the database.
pub struct UserEntry {
    pub user: UserAccount,
    pub password: String,
    pub salt: String,
}

impl From<UserEntry> for UserAccount {
    fn from(entry: UserEntry) -> Self {
        entry.user
    }
}

/// It includes just the user's password and salt.
pub struct UserPasswordSalt {
    pub password: String,
    pub salt: String,
}
