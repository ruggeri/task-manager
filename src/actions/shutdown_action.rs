use super::{Action, ActionResult};
use components::Reviewer;

pub struct ShutdownAction();

impl Action for ShutdownAction {
  fn execute(&mut self, _reviewer: &Reviewer) -> ActionResult {
    ActionResult::RequestedShutDown
  }

  fn unexecute(&mut self, _reviewer: &Reviewer) -> ActionResult {
    panic!("Should not try to undo a ShutdownAction")
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
