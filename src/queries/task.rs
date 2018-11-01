// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Task, TaskDuration, TaskPriority, TaskStatus};
use schema::tasks;

#[derive(Insertable)]
#[table_name = "tasks"]
struct NewTask {
  pub title: String,
  pub status: TaskStatus,
}

pub fn all(connection: &PgConnection) -> Vec<Task> {
  use schema::tasks::dsl::*;
  tasks
    .filter(status.eq(TaskStatus::AvailableToPerform))
    .order(id)
    .load::<Task>(connection)
    .unwrap()
}

pub fn create(title: &str, connection: &PgConnection) -> Task {
  let new_task = NewTask {
    title: String::from(title),
    status: TaskStatus::AvailableToPerform,
  };

  diesel::insert_into(::schema::tasks::table)
    .values(&new_task)
    .get_result(connection)
    .expect("Error creating task")
}

pub fn destroy(task_id: i32, connection: &PgConnection) {
  {
    use schema::task_efforts::dsl;
    diesel::delete(dsl::task_efforts.filter(dsl::task_id.eq(task_id)))
      .execute(connection)
      .expect("Error destroying task");
  }

  use schema::tasks::dsl::*;
  let num_deleted = diesel::delete(tasks.find(task_id))
    .execute(connection)
    .expect("Error destroying task");

  if num_deleted != 1 {
    panic!("Expected to destroy exactly one task");
  }
}

pub fn update_requires_internet(task_id: i32, new_value: bool, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task_id))
    .set(requires_internet.eq(new_value))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }
}

pub fn update_status(task_id: i32, new_status: TaskStatus, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task_id))
    .set(status.eq(new_status))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }
}

pub fn update_title(task_id: i32, new_title: &str, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task_id))
    .set(title.eq(new_title))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }
}

pub fn update_duration(task_id: i32, new_duration: TaskDuration, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task_id))
    .set(duration.eq(new_duration))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }
}

pub fn update_priority(task_id: i32, new_priority: TaskPriority, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task_id))
    .set(priority.eq(new_priority))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }
}
