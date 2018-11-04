// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Task, TaskEvent, TaskEventType};
use schema::task_events;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Insertable)]
#[table_name = "task_events"]
struct NewTaskEvent {
  pub task_id: i32,
  pub event_type: TaskEventType,
}

pub fn last_effort_at(task: &Task, connection: &PgConnection) -> Option<DateTime> {
  use schema::task_events::dsl::*;

  let te = TaskEvent::belonging_to(task)
    .filter(
      destroyed.eq(false)
      .and(event_type.eq(TaskEventType::TaskEffortRecorded))
    )
    .order(created_at.desc())
    .first::<TaskEvent>(connection)
    .optional()
    .unwrap();

  match te {
    None => None,
    Some(te) => Some(te.created_at),
  }
}

pub fn record_task_effort(task_id: i32, connection: &PgConnection) -> TaskEvent {
  let new_te = NewTaskEvent {
    task_id,
    event_type: TaskEventType::TaskEffortRecorded,
  };

  diesel::insert_into(::schema::task_events::table)
    .values(&new_te)
    .get_result(connection)
    .unwrap()
}

define_update_attribute_fns!(
  task_events,
  (update_destroyed, bool, destroyed)
);
