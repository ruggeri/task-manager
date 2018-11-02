use super::{Action, ActionRequest::RequestShutDown};
use components::Reviewer;

pub struct ShutdownAction();

impl Action for ShutdownAction {
  fn execute(&mut self, reviewer: &Reviewer) {
    reviewer.execute_action_request(RequestShutDown);
  }

  fn unexecute(&mut self, _reviewer: &Reviewer) {
    panic!("Should not try to undo a ShutdownAction");
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
