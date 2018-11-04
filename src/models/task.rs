// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use models::{TaskDuration, TaskPriority, TaskStatus};
use schema::tasks;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Clone, Debug, Identifiable, Queryable)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub status: TaskStatus,
  pub created_at: DateTime,
  pub requires_internet: bool,
  pub priority: TaskPriority,
  pub duration: TaskDuration,
  pub destroyed: bool,
}
