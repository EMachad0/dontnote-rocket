pub mod note;
pub mod user;

pub trait Model {
    const TABLE: &'static str;
}
