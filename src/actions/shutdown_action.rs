use super::{Action, ActionRequest::RequestShutDown};
use application::Application;

pub struct ShutdownAction();

impl Action for ShutdownAction {
  fn execute(&mut self, application: &Application) {
    application.execute_action_request(RequestShutDown);
  }

  fn unexecute(&mut self, _application: &Application) {
    panic!("Should not try to undo a ShutdownAction");
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
