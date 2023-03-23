mod user;

#[derive(MergedObject, Default)]
pub struct ResolverRoot(user::UserQuery);
