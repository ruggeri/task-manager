// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use models::{Task, TaskEventType};
use schema::task_events;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
#[belongs_to(Task)]
pub struct TaskEvent {
  pub id: i32,
  pub task_id: i32,
  pub created_at: DateTime,
  pub destroyed: bool,
  pub event_type: TaskEventType,
}
