use actions::ForwardAction;
use components::Scroller;
use models::{Direction, End};
use std::rc::Rc;

pub enum ScrollAction {
  Jump {
    end: End,
    scroller: Rc<Scroller>,
  },
  JumpToTask {
    task_id: i32,
    scroller: Rc<Scroller>,
  },
  Scroll {
    direction: Direction,
    scroller: Rc<Scroller>,
  }
}

impl ForwardAction for ScrollAction {
  fn execute(&mut self) {
    use self::ScrollAction::*;

    match self {
      Jump { end, scroller } => {
        scroller.jump(*end)
      }
      JumpToTask { task_id, scroller} => {
        scroller.jump_to_task_id(task_id)
      }
      Scroll { direction, scroller } => {
        scroller.scroll(*direction)
      }
    }
  }
}
