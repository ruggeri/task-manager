// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

use super::task::Task;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{TaskDuration, TaskPriority, TaskStatus};
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

pub fn create(title: String, connection: &PgConnection) -> Task {
  let new_task = NewTask {
    title,
    status: TaskStatus::AvailableToPerform,
  };

  diesel::insert_into(::schema::tasks::table)
    .values(&new_task)
    .get_result(connection)
    .expect("Error creating task")
}

pub fn destroy(task: &mut Task, connection: &PgConnection) {
  {
    use schema::task_efforts::dsl::*;
    diesel::delete(task_efforts.filter(task_id.eq(task.id)))
      .execute(connection)
      .expect("Error destroying task");
  }

  use schema::tasks::dsl::*;
  let num_deleted = diesel::delete(tasks.find(task.id))
    .execute(connection)
    .expect("Error destroying task");

  if num_deleted != 1 {
    panic!("Expected to destroy exactly one task");
  }
}

pub fn toggle_internet(task: &mut Task, connection: &PgConnection) {
  use diesel::dsl::*;
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task.id))
    .set(requires_internet.eq(not(requires_internet)))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }
}

pub fn update_status(task: &mut Task, new_status: TaskStatus, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task.id))
    .set(status.eq(new_status))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }

  task.status = new_status;
}

pub fn update_title(task: &mut Task, new_title: &str, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task.id))
    .set(title.eq(new_title))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }

  task.title = String::from(new_title);
}

pub fn update_duration(task: &mut Task, new_duration: TaskDuration, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task.id))
    .set(duration.eq(new_duration))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }

  task.duration = new_duration;
}

pub fn update_priority(task: &mut Task, new_priority: TaskPriority, connection: &PgConnection) {
  use schema::tasks::dsl::*;

  let num_updated = diesel::update(tasks.find(task.id))
    .set(priority.eq(new_priority))
    .execute(connection)
    .expect("Error updating task");

  if num_updated != 1 {
    panic!("Expected to update exactly one task");
  }

  task.priority = new_priority;
}
