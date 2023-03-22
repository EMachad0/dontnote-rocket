use serde::{Deserialize, Serialize};

use crate::model::Model;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Model for User {
    const TABLE: &'static str = "user";
}
