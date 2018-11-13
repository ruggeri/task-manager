use actions::ForwardAction;
use components::{Scroller, TasksScroller};
use models::{Direction, End};
use std::rc::Rc;

#[derive(Clone)]
pub enum ScrollAction {
  Jump {
    end: End,
    scroller: Rc<Scroller>,
  },
  Scroll {
    direction: Direction,
    scroller: Rc<Scroller>,
  },
}

#[derive(Clone)]
pub enum TasksScrollAction {
  JumpToTask {
    task_id: i32,
    scroller: Rc<TasksScroller>,
  },
}

impl ForwardAction for ScrollAction {
  fn execute(&mut self) {
    use self::ScrollAction::*;

    match self {
      Jump { end, scroller } => scroller.jump(*end),
      Scroll {
        direction,
        scroller,
      } => scroller.scroll(*direction),
    }
  }
}

impl ForwardAction for TasksScrollAction {
  fn execute(&mut self) {
    use self::TasksScrollAction::*;

    match self {
      JumpToTask { task_id, scroller } => {
        scroller.jump_to_task_id(*task_id);
      }
    }
  }
}
