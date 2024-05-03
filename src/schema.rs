// @generated automatically by Diesel CLI.

diesel::table! {
    receipt (id) {
        id -> Uuid,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        sum -> Numeric,
    }
}

diesel::table! {
    receipt_item (id) {
        id -> Uuid,
        created_at -> Timestamp,
        last_modified_at -> Timestamp,
        #[max_length = 255]
        name -> Varchar,
        amount -> Int4,
        price -> Numeric,
        receipt_id -> Uuid,
    }
}

diesel::joinable!(receipt_item -> receipt (receipt_id));

diesel::allow_tables_to_appear_in_same_query!(receipt, receipt_item,);
