use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::response::content::RawHtml;
use rocket::State;

use crate::auth::AuthSubject;
use crate::database::Database;
use crate::graphql::Schema;
use crate::model::user::User;

#[rocket::get("/")]
pub fn graphiql() -> RawHtml<String> {
    let graphiql = GraphiQLSource::build().endpoint("/graphql").finish();
    RawHtml(graphiql)
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<Schema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema.inner()).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<Schema>,
    db: &State<Database>,
    subject: AuthSubject,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let user = match Some(subject) {
        None => None,
        Some(subject) => {
            let id = subject.as_str().split_once(':').unwrap();
            println!("{:?}", id);
            let user: Vec<User> = db.select(("user", "oqzqfhvcoslpaqa7qbu2")).await.unwrap();
            println!("{:?}", user);
            Some(user[0].clone())
        }
    };
    if let Some(user) = user {
        request.data(user).execute(schema.inner()).await
    } else {
        request.execute(schema.inner()).await
    }
}
