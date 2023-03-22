mod user_login;
mod user_register;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    user_register::UserRegisterMutation,
    user_login::UserLoginMutation,
);
