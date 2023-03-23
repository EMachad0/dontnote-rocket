use diesel::prelude::*;

use crate::database::Database;
use crate::model::user::User;

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<User>> {
        let current_user = User::from_context(ctx)?;

        let users: Vec<User> = {
            use crate::schema::users::dsl::users;
            let mut conn = Database::from_context(ctx).get()?;
            users.find(current_user.id).load(&mut conn)?
        };
        Ok(users)
    }
}
