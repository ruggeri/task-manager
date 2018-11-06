use actions::{Action, ShutdownAction};
use commands::{FiltererCommand, ScrollCommand, TaskCommand, TaskUpdateCommand, UndoBufferCommand};
use application::Application;
use models::{Direction, End, TaskStatus};

#[derive(Clone, Copy, Debug)]
pub enum Command {
  Filter(FiltererCommand),
  Scroll(ScrollCommand),
  Shutdown,
  Task(TaskCommand),
  Undo(UndoBufferCommand),
}

impl Command {
  pub fn from_key(ch: char) -> Option<Command> {
    use self::{
      Command::*, Direction::*, End::*, FiltererCommand::*, ScrollCommand::*, TaskCommand::*, TaskStatus::*,
      TaskUpdateCommand::*,
    };

    let command = match ch {
      'F' => Filter(FilterByRequiresInternet),
      '$' => Scroll(Jump(Bottom)),
      'g' => Scroll(Jump(Top)),
      '/' => Scroll(JumpToTask),
      'k' => Scroll(Move(Decrease)),
      'j' => Scroll(Move(Increase)),
      'q' => Shutdown,
      'n' => Task(CreateTask),
      'r' => Task(RecordTaskEffort),
      'l' => Task(RequestTaskDelay),
      'e' => Task(UpdateTask(EditTaskTitle)),
      'i' => Task(UpdateTask(ToggleRequiresInternet)),
      'd' => Task(UpdateTask(UpdateDuration(Decrease))),
      'D' => Task(UpdateTask(UpdateDuration(Increase))),
      'p' => Task(UpdateTask(UpdatePriority(Decrease))),
      'P' => Task(UpdateTask(UpdatePriority(Increase))),
      'a' => Task(UpdateTask(UpdateStatus(Abandoned))),
      'c' => Task(UpdateTask(UpdateStatus(Completed))),
      'U' => Undo(UndoBufferCommand::Redo),
      'u' => Undo(UndoBufferCommand::Undo),
      _ => return None,
    };

    Some(command)
  }

  pub fn to_action(self, application: &mut Application) -> Option<Box<dyn Action>> {
    use self::Command::*;

    match self {
      Filter(fc) => fc.to_action(application),
      Scroll(sc) => Some(Box::new(sc)),
      Shutdown => Some(Box::new(ShutdownAction())),
      Task(tc) => tc.to_action(application),
      Undo(ubc) => Some(Box::new(ubc)),
    }
  }
}
