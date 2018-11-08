use components::data_source;
use std::cell::RefCell;
use std::rc::Rc;

type ResultsVec = Rc<Vec<data_source::Result>>;
type Callback = dyn Fn(&ResultsVec, FiltererEvent) -> ();

#[derive(Clone, Copy)]
pub enum FiltererEvent {
  FiltererCriteriaUpdated,
  FiltererGotUpdatedResults,
}

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

#[derive(Clone, Debug, Default)]
pub struct FiltererState {
  requires_internet_value: RequiresInternetFiltererValue,
  results: ResultsVec,
}

#[derive(Default)]
pub struct Filterer {
  pub state: RefCell<FiltererState>,
  callbacks: Vec<Box<Callback>>,
}

impl Filterer {
  pub fn new() -> Filterer {
    let state = FiltererState {
      requires_internet_value: RequiresInternetFiltererValue::Any,
      results: Rc::new(vec![]),
    };

    Filterer {
      state: RefCell::new(state),
      callbacks: vec![],
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

  pub fn push(&self, event: FiltererEvent) {
    let state = self.state.borrow();
    for callback in &self.callbacks {
      callback(&state.results, event);
    }
  }

  pub fn refresh(&self, results: &ResultsVec) {
    let filtered_results: Vec<data_source::Result> = results
      .iter()
      .filter(|result| self.filter_result(result))
      .cloned()
      .collect();

    {
      let filtered_results = Rc::new(filtered_results);
      self.state.borrow_mut().results = filtered_results;
    }
    self.push(FiltererEvent::FiltererGotUpdatedResults);
  }

  pub fn requires_internet_value(&self) -> RequiresInternetFiltererValue {
    self.state.borrow().requires_internet_value
  }

  pub fn set_requires_internet_value(&self, new_value: RequiresInternetFiltererValue) {
    {
      let mut state = self.state.borrow_mut();
      state.requires_internet_value = new_value;
    }

    self.push(FiltererEvent::FiltererCriteriaUpdated);
  }
}
