use crate::database::Database;
use crate::model::user::User;
use crate::model::Model;

#[derive(InputObject)]
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

#[derive(Default)]
pub struct UserRegisterMutation;

#[Object]
impl UserRegisterMutation {
    async fn user_register(
        &self,
        ctx: &async_graphql::Context<'_>,
        input: UserInput,
    ) -> async_graphql::Result<User> {
        let db = ctx.data::<Database>()?;
        let user = db.create(User::TABLE).content(User::from(input)).await?;
        Ok(user)
    }
}
