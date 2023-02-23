use tokio::sync::RwLock;

use crate::database::SurrealDb;

pub struct Context {
    pub db: RwLock<SurrealDb>,
}

impl Context {
    pub async fn new() -> anyhow::Result<Self> {
        let db = crate::database::init_db().await?;
        Ok(Self {
            db: RwLock::new(db),
        })
    }
}

impl juniper::Context for Context {}
