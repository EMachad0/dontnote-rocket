mod auth;
mod database;
mod graphql;
mod model;
mod schema;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate async_graphql;

#[macro_use]
extern crate thiserror;

fn build_rocket(db: database::Database) -> rocket::Rocket<rocket::Build> {
    let auth = auth::Auth::new(&std::env::var("SECRET").expect("env var SECRET not set"));

    let schema = async_graphql::Schema::build(
        graphql::ResolverRoot::default(),
        graphql::MutationRoot::default(),
        graphql::SubscriptionRoot,
    )
    .data(auth.clone())
    .data(db.clone())
    .finish();

    rocket::build()
        .manage(schema)
        .manage(auth)
        .manage(db)
        .mount(
            "/",
            routes![
                graphql::graphiql,
                graphql::graphql_query,
                graphql::graphql_request,
            ],
        )
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let db_url = std::env::var("DATABASE_URL")?;
    let db = database::Database::new(&db_url)?;

    let _rocket = build_rocket(db).launch().await?;

    Ok(())
}
