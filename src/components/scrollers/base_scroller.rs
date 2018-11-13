use super::{ScrollerEvent, Scroller, ScrollerState};
use models::{Direction, End};
use std::cell::RefCell;
use std::rc::Rc;

pub type Callback<ResultType> = dyn Fn(ScrollerEvent<ResultType>) -> ();
pub type ResultsVec<ResultType> = Rc<Vec<ResultType>>;

pub struct BaseScroller<ResultType: Clone> {
  pub(super) state: RefCell<ScrollerState<ResultType>>,
  pub(super) callbacks: Vec<Box<Callback<ResultType>>>,
}

impl<ResultType: Clone> BaseScroller<ResultType> {
  pub fn new() -> BaseScroller<ResultType> {
    let state = ScrollerState {
      current_result_idx: 0,
      results: Rc::new(vec![]),
    };

    BaseScroller {
      state: RefCell::new(state),
      callbacks: vec![],
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback<ResultType>>) {
    self.callbacks.push(callback);
  }

  pub fn current_result_idx(&self) -> i32 {
    let state = self.state.borrow();
    state.current_result_idx
  }

  pub fn jump(&self, end: End) {
    let old_result_idx = self.current_result_idx();
    match end {
      End::Top => self._set_current_result_idx(0),
      End::Bottom => {
        self._set_current_result_idx(self.num_results() - 1)
      }
    }

    self._push(ScrollerEvent::ChangedScrollPosition {
      old_result_idx,
      new_state: self.state.borrow().clone(),
    });
  }

  pub fn num_results(&self) -> i32 {
    self.results().len() as i32
  }

  pub(super) fn _push(&self, event: ScrollerEvent<ResultType>) {
    for callback in &self.callbacks {
      callback(event.clone());
    }
  }

  pub fn refresh(&self, results: &ResultsVec<ResultType>) {
    let old_result_idx = self.current_result_idx();

    {
      let mut state = self.state.borrow_mut();
      state.results = Rc::clone(results);
    }

    self._set_current_result_idx(old_result_idx);

    // Push changes on down the line.
    self._push(ScrollerEvent::GotNewScrollResults {
      state: self.state.borrow().clone(),
    });
  }

  pub fn results(&self) -> ResultsVec<ResultType> {
    Rc::clone(&self.state.borrow().results)
  }

  pub fn scroll(&self, direction: Direction) {
    let old_result_idx = self.current_result_idx();
    match direction {
      Direction::Decrease => {
        self._set_current_result_idx(old_result_idx - 1)
      }
      Direction::Increase => {
        self._set_current_result_idx(old_result_idx + 1)
      }
    };

    self._push(ScrollerEvent::ChangedScrollPosition {
      old_result_idx,
      new_state: self.state.borrow().clone(),
    });
  }

  // Only "subclassers" should be using this internal method. Doesn't
  // fire events or call callbacks.
  pub(super) fn _set_current_result_idx(&self, mut new_result_idx: i32) {
    // Check for scrolling off either end.
    let num_results = self.num_results();
    if new_result_idx < 0 {
      new_result_idx = 0;
    } else if new_result_idx >= num_results {
      new_result_idx = num_results - 1;
    }

    self.state.borrow_mut().current_result_idx = new_result_idx;
  }
}

impl<ResultType: Clone> Default for BaseScroller<ResultType> {
  fn default() -> BaseScroller<ResultType> {
    BaseScroller::new()
  }
}

impl<ResultType: Clone> Scroller for BaseScroller<ResultType> {
  fn current_result_idx(&self) -> i32 {
    BaseScroller::<ResultType>::current_result_idx(self)
  }

  fn jump(&self, end: End) {
    BaseScroller::<ResultType>::jump(self, end)
  }

  fn num_results(&self) -> i32 {
    BaseScroller::<ResultType>::num_results(self)
  }

  fn scroll(&self, direction: Direction) {
    BaseScroller::<ResultType>::scroll(self, direction)
  }
}
