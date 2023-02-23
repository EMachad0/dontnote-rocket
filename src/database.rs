use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type SurrealDb = Surreal<Client>;

pub struct Database(RwLock<SurrealDb>);

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        let db = init_db().await?;
        Ok(Database(RwLock::new(db)))
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, SurrealDb> {
        self.0.read().await
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, SurrealDb> {
        self.0.write().await
    }
}

pub(crate) async fn init_db() -> surrealdb::Result<SurrealDb> {
    let db = Surreal::new::<Ws>("localhost:8432").await?;
    println!("db: {:?}", db);

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    println!("db: {:?}", db);

    db.use_ns("dev").use_db("dontnote").await?;
    println!("db: {:?}", db);

    Ok(db)
}
