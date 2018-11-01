// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use chrono::Duration;
use diesel::pg::PgConnection;
use models::{TaskDuration, TaskPriority, TaskStatus};
use queries::task_effort as te_queries;
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
}

impl Task {
  pub fn last_effort_at(&self, connection: &PgConnection) -> Option<DateTime> {
    te_queries::last_effort_at(self, connection)
  }

  pub fn age_at(&self, current_time: DateTime, connection: &PgConnection) -> Duration {
    let last_effort_at = match self.last_effort_at(connection) {
      None => self.created_at,
      Some(t) => t,
    };

    current_time.signed_duration_since(last_effort_at)
  }
}
