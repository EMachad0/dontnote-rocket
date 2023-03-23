use diesel::Queryable;
use uuid::Uuid;

#[derive(Debug, Clone, SimpleObject, Queryable)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}
