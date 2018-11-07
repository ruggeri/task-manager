use components::data_source;
use std::cell::Cell;
use std::rc::Rc;

type ResultsVec = Rc<Vec<data_source::Result>>;
type Callback = dyn Fn(&ResultsVec) -> ();

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequiresInternetFiltererValue {
  Any,
  No,
  Yes,
}

impl Default for RequiresInternetFiltererValue {
  fn default() -> RequiresInternetFiltererValue {
    RequiresInternetFiltererValue::Any
  }
}

#[derive(Default)]
pub struct AttributeFilter {
  pub requires_internet_value: Cell<RequiresInternetFiltererValue>,
  callbacks: Vec<Box<Callback>>,
}

impl AttributeFilter {
  pub fn new() -> AttributeFilter {
    AttributeFilter {
      requires_internet_value: Cell::new(RequiresInternetFiltererValue::Any),
      callbacks: vec![]
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn filter_result(&self, result: &data_source::Result) -> bool {
    use self::RequiresInternetFiltererValue::*;

    match self.requires_internet_value.get() {
      Any => true,
      No => !result.task.requires_internet,
      Yes => result.task.requires_internet,
    }
  }

  pub fn refresh(&self, results: &ResultsVec) {
    let filtered_results: Vec<data_source::Result> = results.iter().filter(|result| {
      self.filter_result(result)
    }).cloned().collect();

    let filtered_results = Rc::new(filtered_results);
    for callback in &self.callbacks {
      callback(&filtered_results);
    }
  }
}
