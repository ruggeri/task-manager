use super::data_source::TaskResult;
use models::Task;
use std::cell::{Cell, Ref, RefCell};

pub struct Scroller {
  pub current_result_idx: Cell<usize>,
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

      let current_task_id = self.current_task().unwrap().id;
      self.current_task_id.set(Some(current_task_id));
    }
  }

  pub fn scroll_backward(&self) {
    let current_result_idx = self.current_result_idx();
    if current_result_idx > 0 {
      self.current_result_idx.set(current_result_idx - 1);

      let current_task_id = self.current_task().unwrap().id;
      self.current_task_id.set(Some(current_task_id));
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

    // First try to match to prev task's id. Find that idx.
    let prev_task_id = self.current_task_id();
    if let Some(prev_task_id) = prev_task_id {
      let new_result_idx = self
        .results()
        .iter()
        .position(|tr| tr.task.id == prev_task_id);

      if let Some(new_result_idx) = new_result_idx {
        self.current_result_idx.set(new_result_idx);
        return
      }
    }

    // If prev task was removed, maintain position.
    let prev_result_idx = self.current_result_idx();
    let num_results = self.results().len();

    let (new_result_idx, new_task_id) = if num_results == 0 {
      (0, None)
    } else if prev_result_idx >= num_results {
      let task_id = self.results().last().unwrap().task.id;
      (num_results - 1, Some(task_id))
    } else {
      let task_id = self.results()[prev_result_idx].task.id;
      (prev_result_idx, Some(task_id))
    };

    self.current_result_idx.set(new_result_idx);
    self.current_task_id.set(new_task_id);
  }
}
