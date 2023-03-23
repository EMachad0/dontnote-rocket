// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}
