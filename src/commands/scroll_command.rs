use actions::ScrollAction;
use components::Scroller;
use models::{Direction, End};
use std::rc::Rc;
use util::ui::Window;

#[derive(Clone, Copy, Debug)]
pub enum ScrollCommand {
  Jump(End),
  JumpToTask,
  Scroll(Direction),
}

impl ScrollCommand {
  pub fn to_action(self, window: &Window, scroller: &Rc<Scroller>) -> Option<ScrollAction> {
    match self {
      ScrollCommand::Jump(end) => {
        Some(ScrollAction::Jump {
          end,
          scroller: Rc::clone(scroller),
        })
      }
      ScrollCommand::JumpToTask => {
        read_task_to_jump_to(window).map(|task_id| {
          ScrollAction::JumpToTask { task_id, scroller: Rc::clone(scroller) }
        })
      }
      ScrollCommand::Scroll(direction) => {
        Some(ScrollAction::Scroll {
          direction,
          scroller: Rc::clone(scroller),
        })
      }
    }
  }
}

fn read_task_to_jump_to(window: &Window) -> Option<i32> {
  let task_id_str = match window.read_line("Task id to jump to: ") {
    None => return None,
    Some(task_id_str) => task_id_str
  };

  task_id_str.parse().ok()
}
