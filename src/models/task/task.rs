// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use chrono::{DateTime, Duration, Utc};
use diesel::pg::PgConnection;
use ::models::{TaskEffort, TaskStatus};
use ::schema::tasks;
use super::queries;

#[derive(Debug, Identifiable, Queryable)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub status: TaskStatus,
  pub created_at: DateTime<Utc>,
}

impl Task {
  pub fn all(connection: &PgConnection) -> Vec<Task> {
    queries::all(connection)
  }

  pub fn create(connection: &PgConnection, title: String) -> Task {
    queries::create(connection, title)
  }

  pub fn abandon(&mut self, connection: &PgConnection) {
    self.update_status(TaskStatus::Abandoned, connection)
  }

  pub fn destroy(self, connection: &PgConnection) {
    queries::destroy(self, connection)
  }

  pub fn last_effort_at(&self, connection: &PgConnection) -> Option<DateTime<Utc>> {
    TaskEffort::last_effort_at(self, connection)
  }

  pub fn mark_completed(&mut self, connection: &PgConnection) {
    self.update_status(TaskStatus::Completed, connection)
  }

  pub fn record_effort(&self, connection: &PgConnection) -> TaskEffort {
    TaskEffort::record_effort(self, connection)
  }

  pub fn age_at(&self, current_time: DateTime<Utc>, connection: &PgConnection) -> Duration {
    let last_effort_at = match self.last_effort_at(connection) {
      None => self.created_at,
      Some(t) => t,
    };

    current_time.signed_duration_since(last_effort_at)
  }

  pub fn update_status(&mut self, status: TaskStatus, connection: &PgConnection) {
    queries::update_status(self, status, connection)
  }
}
