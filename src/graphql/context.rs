use std::sync::RwLock;

use crate::database::Database;

pub struct Context {
    pub db: RwLock<Database>,
}

impl juniper::Context for Context {}
