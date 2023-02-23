use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse, GraphQLQuery};
use rocket::response::content::RawHtml;
use rocket::State;

use super::Schema;

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
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute(schema.inner()).await
}
