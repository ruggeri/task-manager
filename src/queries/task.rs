// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Task, TaskDuration, TaskPriority, TaskStatus};
use schema::tasks;

#[derive(Insertable)]
#[table_name = "tasks"]
struct NewTask {
  title: String,
  status: TaskStatus,
}

pub fn all_available_to_perform(
  connection: &PgConnection,
) -> Vec<Task> {
  use schema::tasks::dsl::*;
  tasks
    .filter(
      status
        .eq(TaskStatus::AvailableToPerform)
        .and(destroyed.eq(false)),
    ).order(id)
    .load::<Task>(connection)
    .unwrap()
}

pub fn create(title: &str, connection: &PgConnection) -> Task {
  let new_task = NewTask {
    title: String::from(title),
    status: TaskStatus::AvailableToPerform,
  };

  diesel::insert_into(::schema::tasks::table)
    .values(&new_task)
    .get_result(connection)
    .expect("Error creating task")
}

define_update_attribute_fns!(
  tasks,
  (update_requires_internet, bool, requires_internet),
  (update_status, TaskStatus, status),
  (update_title, &str, title),
  (update_duration, TaskDuration, duration),
  (update_priority, TaskPriority, priority),
  (update_destroyed, bool, destroyed)
);
