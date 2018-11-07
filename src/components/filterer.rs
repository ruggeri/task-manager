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

#[derive(Clone, Copy, Debug, Default)]
pub struct FiltererState {
  requires_internet_value: RequiresInternetFiltererValue
}

#[derive(Default)]
pub struct Filterer {
  pub state: Cell<FiltererState>,
  callbacks: Vec<Box<Callback>>,
}

impl Filterer {
  pub fn new() -> Filterer {
    let state = FiltererState {
      requires_internet_value: RequiresInternetFiltererValue::Any
    };

    Filterer {
      state: Cell::new(state),
      callbacks: vec![]
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn filter_result(&self, result: &data_source::Result) -> bool {
    use self::RequiresInternetFiltererValue::*;

    match self.requires_internet_value() {
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

  pub fn requires_internet_value(&self) -> RequiresInternetFiltererValue {
    self.state.get().requires_internet_value
  }

  pub fn set_requires_internet_value(&self, new_value: RequiresInternetFiltererValue) {
    let mut state = self.state.get();
    state.requires_internet_value = new_value;
    self.state.set(state);
  }

  pub fn state(&self) -> FiltererState {
    self.state.get().clone()
  }

  pub fn restore_state(&self, state: FiltererState) {
    self.state.set(state)
  }
}
