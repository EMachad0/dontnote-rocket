mod database;
mod model;
mod graphql;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate juniper;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(graphql::Context {
            db: std::sync::RwLock::new(database::Database::new()),
        })
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
