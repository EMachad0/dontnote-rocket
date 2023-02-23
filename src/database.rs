use surrealdb::engine::remote::ws::Ws;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub type SurrealDb = Surreal<Client>;

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
