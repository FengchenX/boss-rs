table! {
    job (id) {
        id -> Int4,
        creator -> Nullable<Int8>,
        create_at -> Nullable<Int8>,
        update_at -> Nullable<Int8>,
        delete_at -> Nullable<Int8>,
        progress -> Nullable<Float8>,
        status -> Nullable<Varchar>,
        file -> Nullable<Varchar>,
    }
}

table! {
    login_history (id) {
        id -> Int4,
        user_id -> Int4,
        login_timestamp -> Timestamptz,
    }
}

table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        gender -> Bool,
        age -> Int4,
        address -> Varchar,
        phone -> Varchar,
        email -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

joinable!(login_history -> users (user_id));

allow_tables_to_appear_in_same_query!(
    job,
    login_history,
    people,
    users,
);
