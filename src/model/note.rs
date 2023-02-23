use serde::{Deserialize, Serialize};

use crate::model::Model;

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct Note {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub text: String,
}

impl Model for Note {
    const TABLE: &'static str = "note";
}
