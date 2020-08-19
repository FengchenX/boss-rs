table! {
    job (id) {
        id -> Integer,
        creator -> Nullable<Bigint>,
        create_at -> Nullable<Bigint>,
        update_at -> Nullable<Bigint>,
        delete_at -> Nullable<Bigint>,
        progress -> Nullable<Double>,
        status -> Nullable<Varchar>,
        file -> Nullable<Varchar>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    test (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    job,
    posts,
    test,
);
