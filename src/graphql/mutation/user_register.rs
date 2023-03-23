use diesel::prelude::*;

use crate::database::Database;
use crate::model::user::User;

#[derive(InputObject, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: String,
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
        let user: User = {
            use crate::schema::users::dsl::users;
            let mut conn = db.get()?;
            diesel::insert_into(users)
                .values(&input)
                .get_result(&mut conn)?
        };
        Ok(user)
    }
}
