use super::data_source;
use models::{Direction, End, Task};
use std::cell::{Cell, Ref, RefCell};

type ResultsVec = Vec<data_source::Result>;

pub struct Scroller {
  pub current_result_idx: Cell<i32>,
  pub current_task_id: Cell<Option<i32>>,
  pub max_results_to_display: usize,
  pub results: RefCell<ResultsVec>,
}

impl Scroller {
  // TODO: not using max_results_to_display!
  pub fn new(max_results_to_display: usize) -> Scroller {
    Scroller {
      current_result_idx: Cell::new(0),
      current_task_id: Cell::new(None),
      max_results_to_display,
      results: RefCell::new(vec![]),
    }
  }

  pub fn current_task_id(&self) -> Option<i32> {
    self.current_task_id.get()
  }

  pub fn current_result_idx(&self) -> i32 {
    self.current_result_idx.get()
  }

  pub fn set_current_result_idx(&self, mut new_result_idx: i32) {
    // Check for scrolling off either end.
    let num_results = self.num_results();
    if new_result_idx < 0 {
      new_result_idx = 0;
    } else if new_result_idx >= num_results {
      new_result_idx = num_results - 1;
    }

    self.current_result_idx.set(new_result_idx);

    // Reset the current_task_id too.
    let new_task_id = self.current_task().map(|t| t.id);
    self.current_task_id.set(new_task_id);
  }

  // TODO: Bad idea to pass someone outside the class a Ref.
  pub fn results(&self) -> Ref<Vec<data_source::Result>> {
    self.results.borrow()
  }

  pub fn num_results(&self) -> i32 {
    self.results().len() as i32
  }

  pub fn scroll(&self, direction: Direction) {
    let current_result_idx = self.current_result_idx();
    match direction {
      Direction::Decrease => self.set_current_result_idx(current_result_idx - 1),
      Direction::Increase => self.set_current_result_idx(current_result_idx + 1),
    };
  }

  pub fn jump(&self, end: End) {
    match end {
      End::Top => self.set_current_result_idx(0),
      End::Bottom => self.set_current_result_idx(self.num_results() - 1),
    }
  }

  pub fn current_task(&self) -> Option<Task> {
    let idx = self.current_result_idx();
    self.results().get(idx as usize).map(|r| r.task.clone())
  }

  pub fn jump_to_task_id(&self, task_id: i32) -> bool {
    self
      .results()
      .iter()
      .position(|tr| tr.task.id == task_id)
      .map(|result_idx| self.set_current_result_idx(result_idx as i32))
      .is_some()
  }

  pub fn refresh(&self, results: ResultsVec) {
    *self.results.borrow_mut() = results;

    // First try to match to prev task's id. Find that idx.
    let prev_task_id = self.current_task_id();
    if let Some(prev_task_id) = prev_task_id {
      if self.jump_to_task_id(prev_task_id) {
        return;
      }
    }

    // Couldn't find moved task. Maintain current position. We'll
    // deal with falling off the end in the setter.
    let old_result_idx = self.current_result_idx();
    self.set_current_result_idx(old_result_idx);
  }
}
