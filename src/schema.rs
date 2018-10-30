// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

table! {
    task_efforts (id) {
        id -> Int4,
        task_id -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    // Need this to do the mapping from PG enum to Rust enum.
    use diesel::sql_types::*;
    use models::TaskStatusMapping;

    tasks (id) {
        id -> Int4,
        title -> Varchar,
        status -> TaskStatusMapping,
        created_at -> Timestamptz,
    }
}

joinable!(task_efforts -> tasks (task_id));

allow_tables_to_appear_in_same_query!(task_efforts, tasks,);
