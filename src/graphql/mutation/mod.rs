mod user_login;
mod user_register;
mod create_note;

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    user_register::UserRegisterMutation,
    user_login::UserLoginMutation,
    create_note::CreateNoteMutation,
);
