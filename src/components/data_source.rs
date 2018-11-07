use components::Scorer;
use chrono::{Duration, Utc};
use diesel::pg::PgConnection;
use models::{Task, TaskEvent};
use queries::{task as task_queries, task_event as te_queries};
use std::cell::RefCell;
use std::rc::Rc;

type ResultsVec = Rc<Vec<Result>>;
type Callback = dyn Fn(&ResultsVec) -> ();
type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Clone, Debug)]
pub struct Result {
  pub task: Task,
  pub task_events: Vec<TaskEvent>,
  pub last_effort_time: DateTime,
  pub last_effort_duration_since: Duration,
  pub score: i64,
}

#[derive(Clone, Debug)]
pub struct DataSourceState {
  results: Option<ResultsVec>
}

pub struct DataSource {
  state: RefCell<DataSourceState>,
  callbacks: Vec<Box<Callback>>,
}

impl DataSource {
  pub fn new() -> DataSource {
    let state = DataSourceState {
      results: None
    };

    DataSource {
      state: RefCell::new(state),
      callbacks: vec![],
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn pull(&self, connection: &PgConnection) {
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

    {
      let mut state = self.state.borrow_mut();
      state.results = Some(Rc::new(results));
    }
    self.push();
  }

  pub fn push(&self) {
    let state = self.state.borrow();
    let results = match state.results.clone() {
      None => panic!("Why are we pushing with no results?"),
      Some(results) => results
    };

    for callback in &self.callbacks {
      callback(&results);
    }
  }

  pub fn state(&self) -> DataSourceState {
    self.state.borrow().clone()
  }

  pub fn restore_state(&self, state: DataSourceState) {
    *self.state.borrow_mut() = state;
  }
}
