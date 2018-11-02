// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Task, TaskEffort};
use schema::task_efforts;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Insertable)]
#[table_name = "task_efforts"]
struct NewTaskEffort {
  pub task_id: i32,
}

pub fn last_effort_at(task: &Task, connection: &PgConnection) -> Option<DateTime> {
  use schema::task_efforts::dsl::*;

  let te = TaskEffort::belonging_to(task)
    .order(created_at.desc())
    .first::<TaskEffort>(connection)
    .optional()
    .unwrap();

  match te {
    None => None,
    Some(te) => Some(te.created_at),
  }
}

pub fn record(task_id: i32, connection: &PgConnection) -> TaskEffort {
  let new_te = NewTaskEffort { task_id };

  diesel::insert_into(::schema::task_efforts::table)
    .values(&new_te)
    .get_result(connection)
    .unwrap()
}

define_update_attribute_fns!(
  task_efforts,
  (update_destroyed, bool, destroyed)
);
