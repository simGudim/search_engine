table! {
    docmaster (doc_id) {
        doc_id -> Int4,
        doc_name -> Varchar,
        doc_size -> Nullable<Int4>,
        doc_path -> Varchar,
        doc_author -> Nullable<Varchar>,
        doc_description -> Nullable<Varchar>,
        doc_association1 -> Nullable<Varchar>,
        doc_association2 -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    docmaster,
    users,
);
