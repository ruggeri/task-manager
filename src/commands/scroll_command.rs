use components::Reviewer;

use actions::{Action, ActionResult};

#[derive(Debug)]
pub enum ScrollCommand {
  ScrollBackward,
  ScrollForward,
  JumpToBottom,
  JumpToTask,
  JumpToTop,
}

fn jump_to_task(reviewer: &mut Reviewer) {
  let task_id = reviewer.window.read_line("Task id to jump to: ");
  task_id
    .parse()
    .ok()
    .map(|task_id: i32| reviewer.scroller.jump_to_task_id(task_id));
}

impl Action for ScrollCommand {
  fn execute(&mut self, reviewer: &mut Reviewer) -> ActionResult {
    use self::ScrollCommand::*;

    match self {
      ScrollBackward => reviewer.scroller.scroll_backward(),
      ScrollForward => reviewer.scroller.scroll_forward(),
      JumpToBottom => reviewer.scroller.jump_to_bottom(),
      JumpToTask => jump_to_task(reviewer),
      JumpToTop => reviewer.scroller.jump_to_top(),
    }

    ActionResult::DidUpdateScroller
  }

  fn unexecute(&mut self, _reviewer: &mut Reviewer) -> ActionResult {
    panic!("Should not try to undo a scroll action")
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
