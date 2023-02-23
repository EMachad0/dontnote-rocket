use uuid::Uuid;

use crate::graphql::{Context, Mutation, Query};

#[derive(Clone, GraphQLObject)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[graphql_object(context = Context)]
impl Query {
    fn users(context: &Context) -> Vec<User> {
        let db = context.db.read().unwrap();
        db.users.values().cloned().collect()
    }
}

#[graphql_object(context = Context)]
impl Mutation {
    fn create_user(context: &Context, name: String) -> juniper::FieldResult<User> {
        let user = User {
            id: Some(Uuid::new_v4()),
            name,
            email: "".to_string(),
            password: "".to_string(),
        };

        {
            let mut db = context.db.write().unwrap();
            db.users.insert(user.id.unwrap(), user.clone());
        }
        Ok(user)
    }
}
