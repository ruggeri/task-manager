use actions::{Action, ActionRequest::RequestScrollerUpdate};
use application::Application;
use models::{Direction, End};

#[derive(Clone, Copy, Debug)]
pub enum ScrollCommand {
  Jump(End),
  JumpToTask,
  Move(Direction),
}

fn jump_to_task(application: &Application) {
  let task_id_str = match application.window.read_line("Task id to jump to: ") {
    None => return,
    Some(task_id_str) => task_id_str
  };
  task_id_str
    .parse()
    .ok()
    .map(|task_id: i32| application.scroller.jump_to_task_id(task_id));
}

impl Action for ScrollCommand {
  fn execute(&mut self, application: &Application) {
    use self::ScrollCommand::*;

    match self {
      Jump(end) => application.scroller.jump(*end),
      JumpToTask => jump_to_task(application),
      Move(direction) => application.scroller.scroll(*direction),
    }

    application.execute_action_request(RequestScrollerUpdate);
  }

  fn unexecute(&mut self, _application: &Application) {
    panic!("Should not try to undo a scroll action")
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
