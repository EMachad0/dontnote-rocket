pub mod user;
pub mod note;

pub trait Model {
    const TABLE: &'static str;
}
