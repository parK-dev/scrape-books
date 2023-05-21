// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Uuid,
        title -> Varchar,
        price -> Float8,
        in_stock -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
