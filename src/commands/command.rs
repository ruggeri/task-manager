use super::{ScrollCommand, TaskCommand, TaskUpdateCommand};
use actions::{Action, ShutdownAction};
use components::Reviewer;
use models::{Direction, End, TaskStatus};

#[derive(Clone, Copy, Debug)]
pub enum Command {
  Scroll(ScrollCommand),
  Shutdown,
  Task(TaskCommand),
}

impl Command {
  pub fn from_key(ch: char) -> Option<Command> {
    use self::{
      Command::*, Direction::*, End::*, ScrollCommand::*, TaskCommand::*, TaskStatus::*,
      TaskUpdateCommand::*,
    };

    let command = match ch {
      '$' => Scroll(Jump(Bottom)),
      'g' => Scroll(Jump(Top)),
      '/' => Scroll(JumpToTask),
      'k' => Scroll(Move(Decrease)),
      'j' => Scroll(Move(Increase)),
      'q' => Shutdown,
      'n' => Task(CreateTask),
      'r' => Task(RecordTaskEffort),
      'e' => Task(UpdateTask(EditTaskTitle)),
      'i' => Task(UpdateTask(ToggleRequiresInternet)),
      'd' => Task(UpdateTask(UpdateDuration(Decrease))),
      'D' => Task(UpdateTask(UpdateDuration(Increase))),
      'p' => Task(UpdateTask(UpdatePriority(Decrease))),
      'P' => Task(UpdateTask(UpdatePriority(Increase))),
      'a' => Task(UpdateTask(UpdateStatus(Abandoned))),
      'c' => Task(UpdateTask(UpdateStatus(Completed))),
      _ => return None,
    };

    Some(command)
  }

  pub fn to_action(self, reviewer: &mut Reviewer) -> Option<Box<dyn Action>> {
    use self::Command::*;

    match self {
      Scroll(sc) => Some(Box::new(sc)),
      Shutdown => Some(Box::new(ShutdownAction())),
      Task(tc) => tc.to_action(reviewer),
    }
  }
}
