table! {
    secure_user_info (id) {
        id -> Uuid,
        user_id -> Uuid,
        password -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        created_date -> Timestamp,
        username -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    secure_user_info,
    users,
);
