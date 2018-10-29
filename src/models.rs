// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::tasks;

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

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask {
  pub title: String,
  pub status: TaskStatus,
}

impl Task {
  pub fn all(connection: &PgConnection) -> Vec<Task> {
    use super::schema::tasks::dsl::*;
    tasks.load::<Task>(connection).unwrap()
  }

  pub fn create(connection: &PgConnection, title: String) -> Task {
    let new_task = NewTask {
      title: title,
      status: TaskStatus::AvailableToPerform,
    };

    diesel::insert_into(super::schema::tasks::table)
        .values(&new_task)
        .get_result(connection)
        .expect("Error creating task")
  }

  pub fn destroy(&self, connection: &PgConnection) {
    use super::schema::tasks::dsl::*;

    let num_deleted = diesel::delete(tasks.filter(id.eq(self.id)))
        .execute(connection)
        .expect("Error deleting task");

    if num_deleted != 1 {
      panic!("Didn't delete just one task?");
    }
  }
}
