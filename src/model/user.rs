use diesel::Queryable;
use uuid::Uuid;

use crate::auth::AuthError;

#[derive(Debug, SimpleObject, Queryable)]
pub struct User {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
}

impl User {
    pub fn from_context<'ctx>(ctx: &async_graphql::Context<'ctx>) -> anyhow::Result<&'ctx Self> {
        match ctx.data::<User>() {
            Ok(user) => Ok(user),
            Err(e) => Err(AuthError::Unauthenticated.anyhow().context(e.message)),
        }
    }
}
