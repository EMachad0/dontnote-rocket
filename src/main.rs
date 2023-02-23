mod database;
mod graphql;
mod model;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate async_graphql;

fn build_rocket(db: database::Database) -> rocket::Rocket<rocket::Build> {
    let schema = async_graphql::Schema::build(
        graphql::QueryRoot,
        graphql::Mutation,
        async_graphql::EmptySubscription,
    )
    .data(db)
    .finish();

    rocket::build().manage(schema).mount(
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
    println!("trying db");
    let db = database::Database::new().await?;
    println!("conected to db");
    let _rocket = build_rocket(db).launch().await?;
    Ok(())
}
