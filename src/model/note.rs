use diesel::Queryable;
use uuid::Uuid;

#[derive(Debug, SimpleObject, Queryable)]
pub struct Note {
    #[graphql(skip)]
    pub id: i32,
    #[graphql(name = "id")]
    pub uuid: Uuid,
    pub text: String,
}
