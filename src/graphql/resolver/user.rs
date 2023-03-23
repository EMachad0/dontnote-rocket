use diesel::prelude::*;

use crate::database::Database;
use crate::graphql::guards::logged_user::LoggedUserGuard;
use crate::model::user::User;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "LoggedUserGuard")]
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<User>> {
        let current_user = ctx.data::<User>().unwrap();

        let db = ctx.data::<Database>()?;
        let users: Vec<User> = {
            use crate::schema::users::dsl::users;
            let mut conn = db.get()?;
            users.find(current_user.id).load(&mut conn)?
        };
        Ok(users)
    }
}
