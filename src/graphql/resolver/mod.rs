mod users;

#[derive(MergedObject, Default)]
pub struct ResolverRoot(users::UsersQuery);
