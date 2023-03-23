use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use diesel::prelude::*;
use rocket::response::content::RawHtml;
use rocket::State;
use uuid::Uuid;

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
    subject: Option<AuthSubject>,
    mut request: GraphQLRequest,
) -> GraphQLResponse {
    if let Some(subject) = subject {
        if let Ok(uuid) = Uuid::parse_str(subject.as_str()) {
            println!("{:?}", uuid);

            let user: Result<Option<User>, _> = {
                use crate::schema::users::{dsl::users, uuid as uuid_field};
                let mut conn = db.get().unwrap();
                users
                    .filter(uuid_field.eq(uuid))
                    .first(&mut conn)
                    .optional()
            };

            if let Ok(Some(user)) = user {
                request = request.data(user);
            };
        };
    };
    request.execute(schema.inner()).await
}
