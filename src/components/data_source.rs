use components::Scorer;
use chrono::{Duration, Utc};
use diesel::pg::PgConnection;
use models::Task;
use queries::task as task_queries;
use std::rc::Rc;

type Callback = dyn Fn(Vec<Result>) -> ();

#[derive(Clone)]
pub struct Result {
  pub task: Task,
  pub task_age: Duration,
}

pub struct DataSource {
  connection: Rc<PgConnection>,
  callbacks: Vec<Box<Callback>>
}

impl DataSource {
  pub fn new(connection: &Rc<PgConnection>) -> DataSource {
    DataSource {
      connection: Rc::clone(connection),
      callbacks: vec![]
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn refresh(&mut self) {
    let current_time = Utc::now();
    let mut results: Vec<_> = task_queries::all(&self.connection)
      .into_iter()
      .map(|task| {
        let task_age = task.age_at(current_time, &self.connection);
        Result { task, task_age }
      }).collect();

    results.sort_by_key(|t| Scorer::score_task_result(t));
    results.reverse();

    for callback in &self.callbacks {
      // TODO: I'm not happy with how I have to keep cloning Vecs
      // everywhere...
      callback(results.clone());
    }
  }
}
