use juniper::FieldResult;
use serde::{Deserialize, Serialize};

use crate::graphql::{Context, Mutation, Query};
use crate::model::Model;

#[derive(Clone, Serialize, Deserialize, GraphQLObject)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Model for User {
    const TABLE: &'static str = "user";
}

#[derive(GraphQLInputObject)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl From<UserInput> for User {
    fn from(input: UserInput) -> Self {
        Self {
            id: None,
            name: input.name,
            email: input.email,
            password: input.password,
        }
    }
}

#[graphql_object(context = Context)]
impl Query {
    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let db = context.db.read().await;
        let users: Vec<User> = db.select(User::TABLE).await?;
        Ok(users)
    }
}

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_user(context: &Context, input: UserInput) -> FieldResult<User> {
        let db = context.db.read().await;
        let user = db.create(User::TABLE).content(User::from(input)).await?;
        Ok(user)
    }
}
