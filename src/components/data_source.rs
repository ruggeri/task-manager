use components::TaskResult;
use diesel::pg::PgConnection;
use queries::task as task_queries;
use std::cell::RefCell;
use std::rc::Rc;

type ResultsVec = Rc<Vec<TaskResult>>;
type Callback = dyn Fn(&ResultsVec) -> ();

pub struct DataSource {
  results: RefCell<Option<ResultsVec>>,
  callbacks: Vec<Box<Callback>>,
}

impl DataSource {
  pub fn new() -> DataSource {
    DataSource {
      results: RefCell::new(None),
      callbacks: vec![],
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn pull(&self, connection: &PgConnection) {
    let current_time = ::chrono::Utc::now();

    let mut results: Vec<_> =
      task_queries::all_available_to_perform(connection)
        .into_iter()
        .map(|task| {
          TaskResult::from_task(task, current_time, connection)
        }).collect();

    results.sort_by_key(|result| result.score);
    results.reverse();

    {
      *self.results.borrow_mut() = Some(Rc::new(results));
    }
    self.push();
  }

  pub fn push(&self) {
    let results = self.results.borrow().clone();
    let results = match results {
      None => panic!("Why are we pushing with no results?"),
      Some(results) => results,
    };

    for callback in &self.callbacks {
      callback(&results);
    }
  }
}

impl Default for DataSource {
  fn default() -> DataSource {
    DataSource::new()
  }
}
