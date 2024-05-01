// @generated automatically by Diesel CLI.

diesel::table! {
    receipt (id) {
        id -> Uuid,
        sum -> Numeric,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
    }
}
