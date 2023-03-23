use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, SimpleObject, Queryable)]
pub struct Note {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub title: String,
    pub content: String,
    #[graphql(skip)]
    pub user_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::notes)]
pub struct NewNote {
    pub title: String,
    pub content: String,
    pub user_id: i32,
}
