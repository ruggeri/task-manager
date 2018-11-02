use actions::{Action, ActionResult};
use components::Reviewer;
use models::{Direction, End};

#[derive(Clone, Copy, Debug)]
pub enum ScrollCommand {
  Jump(End),
  JumpToTask,
  Move(Direction),
}

fn jump_to_task(reviewer: &Reviewer) {
  let task_id = reviewer.window.read_line("Task id to jump to: ");
  task_id
    .parse()
    .ok()
    .map(|task_id: i32| reviewer.scroller.jump_to_task_id(task_id));
}

impl Action for ScrollCommand {
  fn execute(&mut self, reviewer: &Reviewer) -> ActionResult {
    use self::ScrollCommand::*;

    match self {
      Jump(end) => reviewer.scroller.jump(*end),
      JumpToTask => jump_to_task(reviewer),
      Move(direction) => reviewer.scroller.scroll(*direction),
    }

    ActionResult::DidUpdateScroller
  }

  fn unexecute(&mut self, _reviewer: &Reviewer) -> ActionResult {
    panic!("Should not try to undo a scroll action")
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
