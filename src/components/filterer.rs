use components::data_source;
use std::cell::Cell;

type ResultsVec = Vec<data_source::Result>;
type Callback = dyn Fn(ResultsVec) -> ();

#[derive(Default)]
pub struct AttributeFilter {
  pub requires_internet_value: Cell<Option<bool>>,
  callbacks: Vec<Box<Callback>>,
}

impl AttributeFilter {
  pub fn new() -> AttributeFilter {
    AttributeFilter {
      requires_internet_value: Cell::new(None),
      callbacks: vec![]
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn refresh(&self, results: ResultsVec) {
    for callback in &self.callbacks {
      let filtered_results = results.iter().filter(|result| {
        let requires_internet_value = self.requires_internet_value.get();
        requires_internet_value.map_or(true, |requires_internet_value| {
          result.task.requires_internet == requires_internet_value
        })
      }).cloned().collect();

      callback(filtered_results);
    }
  }
}
