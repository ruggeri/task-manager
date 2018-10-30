// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::{tasks, task_efforts};

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(DbEnum, Debug)]
pub enum TaskStatus {
  Abandoned,
  AvailableToPerform,
  Completed,
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub status: TaskStatus,
  pub created_at: DateTime,
}

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask {
  pub title: String,
  pub status: TaskStatus,
}

#[derive(Associations, Debug, Identifiable, Queryable)]
#[belongs_to(Task)]
pub struct TaskEffort {
  pub id: i32,
  pub task_id: i32,
  pub created_at: DateTime,
}

#[derive(Insertable)]
#[table_name="task_efforts"]
pub struct NewTaskEffort {
  pub task_id: i32,
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

  pub fn last_effort_at(&self, connection: &PgConnection) -> Option<DateTime> {
    use super::schema::task_efforts::dsl::*;

    let te = TaskEffort::belonging_to(self)
      .order(created_at.desc())
      .first::<TaskEffort>(connection).optional().unwrap();

    match te {
      None => None,
      Some(te) => Some(te.created_at)
    }
  }

  pub fn sort_time(&self, connection: &PgConnection) -> DateTime {
    match self.last_effort_at(connection) {
      None => self.created_at,
      Some(t) => t,
    }
  }

  pub fn record_effort(&self, connection: &PgConnection) -> TaskEffort {
    let new_te = NewTaskEffort {
      task_id: self.id,
    };

    let te = diesel::insert_into(super::schema::task_efforts::table)
      .values(&new_te)
      .get_result(connection)
      .unwrap();

    te
  }
}
