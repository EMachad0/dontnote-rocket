use crate::database::Database;
use crate::model::note::Note;
use crate::model::Model;

#[derive(Default)]
pub struct NoteQuery;

#[Object]
impl NoteQuery {
    async fn note(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Vec<Note>> {
        let db = ctx.data::<Database>()?;
        let db = db.read().await;
        let notes: Vec<Note> = db.select(Note::TABLE).await?;
        Ok(notes)
    }
}
