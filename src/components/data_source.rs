use components::Scorer;
use chrono::{Duration, Utc};
use diesel::pg::PgConnection;
use models::{Task, TaskEvent};
use queries::{task as task_queries, task_event as te_queries};

type Callback = dyn Fn(Vec<Result>) -> ();
type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Clone)]
pub struct Result {
  pub task: Task,
  pub task_events: Vec<TaskEvent>,
  pub last_effort_time: DateTime,
  pub last_effort_duration_since: Duration,
  pub score: i64,
}

pub struct DataSource {
  callbacks: Vec<Box<Callback>>
}

impl DataSource {
  pub fn new() -> DataSource {
    DataSource {
      callbacks: vec![]
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn refresh(&self, connection: &PgConnection) {
    let current_time = Utc::now();

    let mut results: Vec<_> = task_queries::all_available_to_perform(&connection)
      .into_iter()
      .map(|task| {
        let task_events = te_queries::task_events(&task, &connection);
        let last_effort_time = Scorer::last_effort_time(&task, &task_events);
        let last_effort_duration_since = current_time.signed_duration_since(last_effort_time);
        let score = Scorer::score_task(&task, &task_events, last_effort_duration_since);

        Result { task, task_events, last_effort_time, last_effort_duration_since, score }
      }).collect();

    results.sort_by_key(|result| result.score);
    results.reverse();

    for callback in &self.callbacks {
      // TODO: I'm not happy with how I have to keep cloning Vecs
      // everywhere...
      callback(results.clone());
    }
  }
}
