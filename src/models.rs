// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

#[derive(DbEnum, Debug)]
pub enum TaskStatus {
  Abandoned,
  AvailableToPerform,
  Completed,
}

#[derive(Debug, Queryable)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub status: TaskStatus,
  pub created_at: ::chrono::DateTime<::chrono::Utc>,
}
