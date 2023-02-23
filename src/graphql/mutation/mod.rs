mod create_user;

#[derive(MergedObject, Default)]
pub struct MutationRoot(create_user::CreateUserMutation);
