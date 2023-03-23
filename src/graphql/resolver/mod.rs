mod users;
mod notes;

#[derive(MergedObject, Default)]
pub struct ResolverRoot(users::UsersQuery, notes::NotesQuery);
