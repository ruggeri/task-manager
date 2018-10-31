use super::data_source::TaskResult;
use models::Task;
use std::cell::{Cell, Ref, RefCell};

pub struct Scroller {
  pub current_result_idx: Cell<i32>,
  pub current_task_id: Cell<Option<i32>>,
  pub max_results_to_display: usize,
  pub results: RefCell<Vec<TaskResult>>,
}

impl Scroller {
  // TODO: not using max_results_to_display!
  pub fn new(results: Vec<TaskResult>, max_results_to_display: usize) -> Scroller {
    Scroller {
      current_result_idx: Cell::new(0),
      current_task_id: Cell::new(None),
      max_results_to_display,
      results: RefCell::new(results),
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
  pub fn results(&self) -> Ref<Vec<TaskResult>> {
    self.results.borrow()
  }

  pub fn num_results(&self) -> i32 {
    self.results().len() as i32
  }

  pub fn scroll_forward(&self) {
    let current_result_idx = self.current_result_idx();
    self.set_current_result_idx(current_result_idx + 1);
  }

  pub fn scroll_backward(&self) {
    let current_result_idx = self.current_result_idx();
    self.set_current_result_idx(current_result_idx - 1);
  }

  pub fn jump_to_top(&self) {
    self.set_current_result_idx(0);
  }

  pub fn jump_to_bottom(&self) {
    self.set_current_result_idx(self.num_results() - 1);
  }

  pub fn current_task(&self) -> Option<Task> {
    let idx = self.current_result_idx();
    self.results().get(idx as usize).map(|r| r.task.clone())
  }

  pub fn refresh(&self, results: Vec<TaskResult>) {
    *self.results.borrow_mut() = results;

    // First try to match to prev task's id. Find that idx.
    let prev_task_id = self.current_task_id();
    let moved_result_idx = if let Some(prev_task_id) = prev_task_id {
      self
        .results()
        .iter()
        .position(|tr| tr.task.id == prev_task_id)
    } else {
      None
    };

    match moved_result_idx {
      Some(moved_result_idx) => {
        // Found moved task; reset idx there.
        self.set_current_result_idx(moved_result_idx as i32);
      }
      None => {
        // Couldn't find moved task. Maintain current position. We'll
        // deal with falling off the end in the setter.
        let old_result_idx = self.current_result_idx();
        self.set_current_result_idx(old_result_idx);
      }
    }
  }
}
