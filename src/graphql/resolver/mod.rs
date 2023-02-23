mod note;
mod user;

#[derive(MergedObject, Default)]
pub struct ResolverRoot(user::UserQuery, note::NoteQuery);
