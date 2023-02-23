use std::collections::HashMap;

pub struct Database {
    pub users: HashMap<uuid::Uuid, crate::model::user::User>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}
