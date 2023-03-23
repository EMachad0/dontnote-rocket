use diesel::prelude::*;

use crate::database::Database;
use crate::model::note::Note;
use crate::model::user::User;

#[derive(Default)]
pub struct NotesQuery;

#[Object]
impl NotesQuery {
    async fn note(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Note>> {
        let current_user = User::from_context(ctx)?;

        let notes: Vec<Note> = {
            use crate::schema::notes::{dsl::notes, user_id};
            let mut conn = Database::from_context(ctx).get()?;
            notes.filter(user_id.eq(current_user.id)).load(&mut conn)?
        };
        Ok(notes)
    }
}
