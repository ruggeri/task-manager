use super::{BaseScroller, Scroller, ScrollerEvent};
use components::TaskResult;
use models::{Direction, End, Task};
use std::rc::Rc;

type Callback = dyn Fn(ScrollerEvent<TaskResult>) -> ();
type ResultsVec = Rc<Vec<TaskResult>>;

pub struct TasksScroller {
  base: BaseScroller<TaskResult>,
}

impl TasksScroller {
  pub fn new() -> TasksScroller {
    TasksScroller {
      base: BaseScroller::new(),
    }
  }

  pub fn add_callback(&mut self, callback: Box<Callback>) {
    self.base.callbacks.push(callback);
  }

  pub fn current_task(&self) -> Option<Task> {
    let idx = self.base.current_result_idx();
    self
      .base
      .results()
      .get(idx as usize)
      .map(|result| result.task.clone())
  }

  pub fn current_task_id(&self) -> Option<i32> {
    let results = self.base.results();

    if results.is_empty() {
      None
    } else {
      Some(results[self.base.current_result_idx() as usize].task.id)
    }
  }

  // This jumps to the task id, but doesn't push an event.
  fn _jump_to_task_id(&self, task_id: i32) -> bool {
    self
      .base
      .results()
      .iter()
      .position(|result| result.task.id == task_id)
      .map(|result_idx| {
        self.base.set_current_result_idx(result_idx as i32)
      }).is_some()
  }

  // This is the method users should call to jump to a task id. It
  // pushes an event down to any followers.
  pub fn jump_to_task_id(&self, task_id: i32) -> bool {
    let old_result_idx = self.base.current_result_idx();
    if self._jump_to_task_id(task_id) {
      self.base.push(ScrollerEvent::ChangedScrollPosition {
        old_result_idx,
        new_state: self.base.state.borrow().clone(),
      });

      true
    } else {
      false
    }
  }

  pub fn refresh(&self, results: &ResultsVec) {
    let old_task_id = self.current_task_id();
    let old_result_idx = self.base.current_result_idx();

    {
      let mut state = self.base.state.borrow_mut();
      state.results = Rc::clone(results);
    }

    self.try_to_maintain_scroll_position(old_task_id, old_result_idx);

    // Push changes on down the line.
    self.base.push(ScrollerEvent::GotNewScrollResults {
      state: self.base.state.borrow().clone(),
    });
  }

  fn try_to_maintain_scroll_position(
    &self,
    old_task_id: Option<i32>,
    old_result_idx: i32,
  ) {
    // First try to match to prev task's id. Find that idx.
    if let Some(old_task_id) = old_task_id {
      if self.jump_to_task_id(old_task_id) {
        return;
      }
    }

    // Couldn't find moved task. Try to at least maintain current
    // position. We'll deal with falling off the end in the setter.
    self.base.set_current_result_idx(old_result_idx);
  }
}

impl Scroller for TasksScroller {
  fn current_result_idx(&self) -> i32 {
    self.base.current_result_idx()
  }

  fn jump(&self, end: End) {
    self.base.jump(end)
  }

  fn num_results(&self) -> i32 {
    self.base.num_results()
  }

  fn scroll(&self, direction: Direction) {
    self.base.scroll(direction)
  }
}

impl Default for TasksScroller {
  fn default() -> TasksScroller {
    TasksScroller::new()
  }
}
