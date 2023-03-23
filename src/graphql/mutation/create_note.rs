use diesel::prelude::*;

use crate::database::Database;
use crate::model::note::{NewNote, Note};
use crate::model::user::User;

#[derive(InputObject)]
pub struct CreateNoteInput {
    pub title: String,
    pub content: String,
}

#[derive(Default)]
pub struct CreateNoteMutation;

#[Object]
impl CreateNoteMutation {
    async fn create_note(
        &self,
        ctx: &async_graphql::Context<'_>,
        input: CreateNoteInput,
    ) -> async_graphql::Result<Note> {
        let current_user = User::from_context(ctx)?;
        let db = ctx.data::<Database>()?;
        let note = {
            use crate::schema::notes::dsl::notes;
            let mut conn = db.get()?;
            diesel::insert_into(notes)
                .values(&NewNote {
                    title: input.title,
                    content: input.content,
                    user_id: current_user.id,
                })
                .get_result(&mut conn)?
        };
        Ok(note)
    }
}
