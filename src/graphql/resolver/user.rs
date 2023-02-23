use crate::database::Database;
use crate::model::user::User;
use crate::model::Model;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<User>> {
        let db = ctx.data::<Database>()?;
        let db = db.read().await;
        let users: Vec<User> = db.select(User::TABLE).await?;
        Ok(users)
    }
}
