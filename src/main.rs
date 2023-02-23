mod database;
mod graphql;
mod model;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate juniper;

fn build_rocket(context: graphql::Context) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .manage(context)
        .manage(graphql::Schema::new(
            graphql::Query,
            graphql::Mutation,
            juniper::EmptySubscription::<graphql::Context>::new(),
        ))
        .mount(
            "/",
            routes![
                graphql::graphiql,
                graphql::get_graphql_handler,
                graphql::post_graphql_handler
            ],
        )
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    println!("trying db");
    let context = graphql::Context::new().await?;
    println!("conected to db");
    let _rocket = build_rocket(context).launch().await?;
    Ok(())
}
