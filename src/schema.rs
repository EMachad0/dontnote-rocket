// @generated automatically by Diesel CLI.

diesel::table! {
    notes (id) {
        id -> Int4,
        uuid -> Uuid,
        title -> Varchar,
        content -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        uuid -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(notes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    notes,
    users,
);
