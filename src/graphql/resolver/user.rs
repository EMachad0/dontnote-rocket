use diesel::prelude::*;

use crate::database::Database;
use crate::model::user::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<User>> {
        let Some(current_user) = ctx.data::<User>().ok() else {
            return Err(async_graphql::Error::new("Not logged in"));
        };

        let db = ctx.data::<Database>()?;
        let users: Vec<User> = {
            use crate::schema::users::dsl::users;
            let mut conn = db.get()?;
            users.find(current_user.id).load(&mut conn)?
        };
        Ok(users)
    }
}
