table! {
    gl_entries (id) {
        id -> Uuid,
        journal_header_id -> Uuid,
        ledger_account_id -> Uuid,
        debit -> Money,
        credit -> Money,
        created_date -> Timestamp,
        modified_date -> Timestamp,
    }
}

table! {
    journal_headers (id) {
        id -> Uuid,
        transaction_amount -> Money,
        created_date -> Timestamp,
        modified_date -> Timestamp,
        memo -> Nullable<Text>,
        cleared -> Bool,
    }
}

table! {
    journal_lines (id) {
        id -> Uuid,
        header_id -> Uuid,
        account_id -> Uuid,
        debit -> Money,
        credit -> Money,
    }
}

table! {
    ledger_accounts (id) {
        id -> Uuid,
        user_id -> Uuid,
        account_name -> Varchar,
        created_date -> Timestamp,
    }
}

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
    gl_entries,
    journal_headers,
    journal_lines,
    ledger_accounts,
    secure_user_info,
    users,
);
