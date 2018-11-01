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
  pub title: String,
  pub status: TaskStatus,
}

pub fn all(connection: &PgConnection) -> Vec<Task> {
  use schema::tasks::dsl::*;
  tasks
    .filter(status.eq(TaskStatus::AvailableToPerform))
    .order(id)
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

// TODO: Restore destroy functionality when safe.
// pub fn destroy(task_id: i32, connection: &PgConnection) {
//   {
//     use schema::task_efforts::dsl;
//     diesel::delete(dsl::task_efforts.filter(dsl::task_id.eq(task_id)))
//       .execute(connection)
//       .expect("Error destroying task");
//   }
//
//   use schema::tasks::dsl::*;
//   let num_deleted = diesel::delete(tasks.find(task_id))
//     .execute(connection)
//     .expect("Error destroying task");
//
//   if num_deleted != 1 {
//     panic!("Expected to destroy exactly one task");
//   }
// }

define_update_attribute_fns!(
  tasks,
  (update_requires_internet, bool, requires_internet),
  (update_status, TaskStatus, status),
  (update_title, &str, title),
  (update_duration, TaskDuration, duration),
  (update_priority, TaskPriority, priority)
);
