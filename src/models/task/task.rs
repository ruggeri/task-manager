// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel::pg::PgConnection;
use ::models::{TaskEffort, TaskStatus};
use ::schema::tasks;
use super::queries;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Debug, Identifiable, Queryable)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub status: TaskStatus,
  pub created_at: DateTime,
}

impl Task {
  pub fn all(connection: &PgConnection) -> Vec<Task> {
    queries::all(connection)
  }

  pub fn create(connection: &PgConnection, title: String) -> Task {
    queries::create(connection, title)
  }

  pub fn abandon(&mut self, connection: &PgConnection) {
    queries::update_status(self, TaskStatus::Abandoned, connection)
  }

  pub fn destroy(self, connection: &PgConnection) {
    queries::destroy(self, connection)
  }

  pub fn mark_completed(&mut self, connection: &PgConnection) {
    queries::update_status(self, TaskStatus::Completed, connection)
  }

  pub fn last_effort_at(&self, connection: &PgConnection) -> Option<DateTime> {
    TaskEffort::last_effort_at(self, connection)
  }

  pub fn sort_time(&self, connection: &PgConnection) -> DateTime {
    match self.last_effort_at(connection) {
      None => self.created_at,
      Some(t) => t,
    }
  }

  pub fn record_effort(&self, connection: &PgConnection) -> TaskEffort {
    TaskEffort::record_effort(self, connection)
  }
}
