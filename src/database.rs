use surrealdb::engine::remote::ws::Client;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub type Database = Surreal<Client>;

pub(crate) async fn init_db() -> surrealdb::Result<Database> {
    let db = Surreal::new::<Ws>("localhost:8432").await?;

    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    db.use_ns("dev").use_db("dontnote").await?;

    Ok(db)
}
