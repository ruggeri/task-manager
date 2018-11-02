// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use models::Task;
use schema::task_efforts;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Associations, Clone, Debug, Identifiable, Queryable)]
#[belongs_to(Task)]
pub struct TaskEffort {
  pub id: i32,
  pub task_id: i32,
  pub created_at: DateTime,
  pub destroyed: bool,
}
