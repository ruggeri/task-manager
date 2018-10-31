use super::data_source::TaskResult;
use models::Task;
use std::cell::{Cell, Ref, RefCell};

pub struct Scroller {
  pub current_result_idx: Cell<usize>,
  pub max_results_to_display: usize,
  pub results: RefCell<Vec<TaskResult>>,
}

impl Scroller {
  pub fn new(results: Vec<TaskResult>, max_results_to_display: usize) -> Scroller {
    Scroller {
      current_result_idx: Cell::new(0),
      max_results_to_display,
      results: RefCell::new(results),
    }
  }

  pub fn current_result_idx(&self) -> usize {
    self.current_result_idx.get()
  }

  // TODO: Bad idea to pass someone outside the class a Ref.
  pub fn results(&self) -> Ref<Vec<TaskResult>> {
    self.results.borrow()
  }

  pub fn scroll_forward(&self) {
    let current_result_idx = self.current_result_idx();
    if current_result_idx < self.results().len() - 1 {
      self.current_result_idx.set(current_result_idx + 1);
    }
  }

  pub fn scroll_backward(&self) {
    let current_result_idx = self.current_result_idx();
    if current_result_idx > 0 {
      self.current_result_idx.set(current_result_idx - 1);
    }
  }

  pub fn current_task(&self) -> Option<Task> {
    if self.results().is_empty() {
      None
    } else {
      let result = self.results()[self.current_result_idx()].clone();
      Some(result.task)
    }
  }

  pub fn refresh(&self, results: Vec<TaskResult>) {
    *self.results.borrow_mut() = results;
    self.fix_index();
  }

  fn fix_index(&self) {
    let num_results = self.results().len();

    if num_results == 0 {
      self.current_result_idx.set(0);
    } else if self.current_result_idx() >= num_results {
      self.current_result_idx.set(num_results - 1);
    }
  }
}
