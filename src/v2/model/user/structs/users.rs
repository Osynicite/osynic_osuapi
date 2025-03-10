use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Users {
    users: Vec<User>,
}
