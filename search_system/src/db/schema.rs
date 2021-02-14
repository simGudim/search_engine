table! {
    users (id) {
        id -> Int4,
        user_id -> Nullable<Uuid>,
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
