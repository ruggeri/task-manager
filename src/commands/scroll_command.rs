use actions::{ScrollAction, TasksScrollAction};
use components::{Scroller, TasksScroller};
use models::{Direction, End};
use std::rc::Rc;
use util::ui::UserInterface;

#[derive(Clone, Copy, Debug)]
pub enum ScrollCommand {
  Jump(End),
  Scroll(Direction),
}

#[derive(Clone, Copy, Debug)]
pub enum TasksScrollCommand {
  JumpToTask,
}

impl ScrollCommand {
  pub fn to_action(
    self,
    scroller: &Rc<Scroller>,
  ) -> Option<ScrollAction> {
    match self {
      ScrollCommand::Jump(end) => Some(ScrollAction::Jump {
        end,
        scroller: Rc::clone(scroller),
      }),
      ScrollCommand::Scroll(direction) => Some(ScrollAction::Scroll {
        direction,
        scroller: Rc::clone(scroller),
      }),
    }
  }
}

impl TasksScrollCommand {
  pub fn to_action(
    self,
    ui: &UserInterface,
    scroller: &Rc<TasksScroller>,
  ) -> Option<TasksScrollAction> {
    match self {
      TasksScrollCommand::JumpToTask => {
        read_task_to_jump_to(ui).map(|task_id| {
          TasksScrollAction::JumpToTask {
            task_id,
            scroller: Rc::clone(scroller),
          }
        })
      }
    }
  }
}

fn read_task_to_jump_to(ui: &UserInterface) -> Option<i32> {
  let task_id_str = match ui.read_line("Task id to jump to: ") {
    None => return None,
    Some(task_id_str) => task_id_str,
  };

  task_id_str.parse().ok()
}
