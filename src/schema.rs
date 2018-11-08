// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

table! {
    use diesel::sql_types::*;
    use models::task_event_type::TaskEventTypeMapping;

    task_events (id) {
        id -> Int4,
        task_id -> Int4,
        created_at -> Timestamptz,
        destroyed -> Bool,
        event_type -> TaskEventTypeMapping,
    }
}

table! {
    // Need this to do the mapping from PG enum to Rust enum.
    use diesel::sql_types::*;
    use models::task_duration::TaskDurationMapping;
    use models::task_priority::TaskPriorityMapping;
    use models::task_status::TaskStatusMapping;

    tasks (id) {
        id -> Int4,
        title -> Varchar,
        status -> TaskStatusMapping,
        created_at -> Timestamptz,
        requires_internet -> Bool,
        priority -> TaskPriorityMapping,
        duration -> TaskDurationMapping,
        destroyed -> Bool,
    }
}

joinable!(task_events -> tasks (task_id));

allow_tables_to_appear_in_same_query!(task_events, tasks,);
