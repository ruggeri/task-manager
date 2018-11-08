use super::data_source;
use models::{Direction, End, Task};
use std::cell::RefCell;
use std::rc::Rc;

type Callback = dyn Fn(&Scroller) -> ();
type ResultsVec = Rc<Vec<data_source::Result>>;

#[derive(Clone, Copy)]
pub enum ScrollerRefreshType {
  MajorRefresh,
  MinorRefresh,
}

#[derive(Clone, Debug)]
pub struct ScrollerState {
  pub current_result_idx: i32,
  pub results: ResultsVec,
}

pub struct Scroller {
  pub state: RefCell<ScrollerState>,
  pub callbacks: Vec<Box<Callback>>
}

impl Scroller {
  pub fn new() -> Scroller {
    let state = ScrollerState {
      current_result_idx: 0,
      results: Rc::new(vec![]),
    };

    Scroller {
      state: RefCell::new(state),
      callbacks: vec![],
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.callbacks.push(callback);
  }

  pub fn current_task_id(&self) -> Option<i32> {
    let results = self.results();
    if results.is_empty() {
      None
    } else {
      Some(results[self.current_result_idx() as usize].task.id)
    }
  }

  pub fn current_result_idx(&self) -> i32 {
    let state = self.state.borrow();
    state.current_result_idx
  }

  fn set_current_result_idx(&self, mut new_result_idx: i32) {
    // Check for scrolling off either end.
    let num_results = self.num_results();
    if new_result_idx < 0 {
      new_result_idx = 0;
    } else if new_result_idx >= num_results {
      new_result_idx = num_results - 1;
    }

    let mut state = self.state.borrow_mut();
    state.current_result_idx = new_result_idx;
  }

  pub fn results(&self) -> Rc<Vec<data_source::Result>> {
    Rc::clone(&self.state.borrow().results)
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

    self.push();
  }

  pub fn jump(&self, end: End) {
    match end {
      End::Top => self.set_current_result_idx(0),
      End::Bottom => self.set_current_result_idx(self.num_results() - 1),
    }

    self.push();
  }

  pub fn current_task(&self) -> Option<Task> {
    let idx = self.current_result_idx();
    self.results().get(idx as usize).map(|r| r.task.clone())
  }

  pub fn jump_to_task_id(&self, task_id: i32) -> bool {
    let result = self._jump_to_task_id(task_id);
    self.push();
    result
  }

  fn _jump_to_task_id(&self, task_id: i32) -> bool {
    self
      .results()
      .iter()
      .position(|tr| tr.task.id == task_id)
      .map(|result_idx| self.set_current_result_idx(result_idx as i32))
      .is_some()
  }

  pub fn refresh(&self, results: &ResultsVec, refresh_type: ScrollerRefreshType) {
    let old_task_id = self.current_task_id();
    let old_result_idx = self.current_result_idx();

    {
      let mut state = self.state.borrow_mut();
      state.results = Rc::clone(results);
    }

    use self::ScrollerRefreshType::*;
    match refresh_type {
      MajorRefresh => self.set_current_result_idx(0),
      MinorRefresh => {
        self.try_to_maintain_scroll_position(
          old_task_id,
          old_result_idx,
        )
      }
    }

    // Push changes on down the line.
    self.push();
  }

  fn try_to_maintain_scroll_position(&self, old_task_id: Option<i32>, old_result_idx: i32) {
    // First try to match to prev task's id. Find that idx.
    if let Some(old_task_id) = old_task_id {
      if self.jump_to_task_id(old_task_id) {
        return;
      }
    }

    // Couldn't find moved task. Try to at least maintain current
    // position. We'll deal with falling off the end in the setter.
    self.set_current_result_idx(old_result_idx);
  }

  pub fn push(&self) {
    for callback in &self.callbacks {
      callback(&self);
    }
  }

  pub fn state(&self) -> ScrollerState {
    self.state.borrow().clone()
  }

  pub fn restore_state(&self, state: ScrollerState) {
    *self.state.borrow_mut() = state;
  }
}

impl Default for Scroller {
  fn default() -> Scroller {
    Scroller::new()
  }
}
