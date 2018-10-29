// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

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

impl Task {
  pub fn all(connection: &PgConnection) -> Vec<Task> {
    use super::schema::tasks::dsl::*;
    tasks.load::<Task>(connection).unwrap()
  }

  pub fn destroy(&self, connection: &PgConnection) {
    use super::schema::tasks::dsl::*;

    let num_deleted = diesel::delete(tasks.filter(id.eq(self.id)))
        .execute(connection)
        .expect("Error deleting posts");

    if num_deleted != 1 {
      panic!("Didn't delete just one task?");
    }
  }
}
