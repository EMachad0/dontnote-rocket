use diesel::prelude::*;

use crate::auth::Auth;
use crate::database::Database;
use crate::model::user::User;

#[derive(Default)]
pub struct UserLoginMutation;

#[Object]
impl UserLoginMutation {
    pub async fn user_login(
        &self,
        ctx: &async_graphql::Context<'_>,
        email: String,
        password: String,
    ) -> async_graphql::Result<bool> {
        let db = ctx.data::<Database>()?;
        let auth = ctx.data::<Auth>()?;
        let user: Option<User> = {
            use crate::schema::users::{dsl::users, email as email_field};
            let mut conn = db.get()?;
            users
                .filter(email_field.eq(email))
                .first(&mut conn)
                .optional()?
        };
        let user = user.ok_or_else(|| async_graphql::Error::new("Invalid username"))?;
        if user.password == password {
            let token = auth.encode_token(&user);
            ctx.insert_http_header(crate::auth::HEADER, token);
            Ok(true)
        } else {
            Err(async_graphql::Error::new("Invalid password"))
        }
    }
}
