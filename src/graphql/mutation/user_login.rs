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
        let mut db_response = db
            .query("SELECT * FROM user WHERE email = $email")
            .bind(("email", email))
            .await?;
        let user: Option<User> = db_response.take(0)?;
        let user = user.ok_or_else(|| async_graphql::Error::new("Invalid username"))?;
        if user.password == password {
            let token = auth.encode_token(&user);
            ctx.insert_http_header(crate::auth::HEADER, &token);
            Ok(true)
        } else {
            Err(async_graphql::Error::new("Invalid password"))
        }
    }
}
