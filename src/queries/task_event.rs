// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Task, TaskEvent, TaskEventType};
use schema::task_events;

#[derive(Insertable)]
#[table_name = "task_events"]
struct NewTaskEvent {
  pub task_id: i32,
  pub event_type: TaskEventType,
}

pub fn task_events(task: &Task, connection: &PgConnection) -> Vec<TaskEvent> {
  use schema::task_events::dsl::*;

  TaskEvent::belonging_to(task)
    .filter(
      destroyed.eq(false)
    )
    .order(created_at.desc())
    .load::<TaskEvent>(connection)
    .unwrap()
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

pub fn request_delay(task_id: i32, connection: &PgConnection) -> TaskEvent {
  let new_te = NewTaskEvent {
    task_id,
    event_type: TaskEventType::DelayRequested,
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
