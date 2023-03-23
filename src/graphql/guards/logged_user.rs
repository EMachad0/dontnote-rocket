use crate::model::user::User;

pub struct LoggedUserGuard;

#[async_trait::async_trait]
impl async_graphql::Guard for LoggedUserGuard {
    async fn check(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<()> {
        match User::from_context(ctx) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
