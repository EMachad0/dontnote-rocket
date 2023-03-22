use crate::database::Database;
use crate::model::user::User;
use crate::model::Model;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<User>> {
        let current_user = ctx.data::<User>()?;
        let db = ctx.data::<Database>()?;
        let users: Vec<User> = db
            .select((User::TABLE, current_user.id.as_ref().unwrap()))
            .await?;
        Ok(users)
    }
}
