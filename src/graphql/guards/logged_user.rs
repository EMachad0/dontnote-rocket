use crate::model::user::User;

pub struct LoggedUserGuard;

#[async_trait::async_trait]
impl async_graphql::Guard for LoggedUserGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        let current_user = ctx.data::<User>().ok();
        match current_user {
            None => Err(async_graphql::Error::new("Not logged in")),
            Some(_) => Ok(()),
        }
    }
}
