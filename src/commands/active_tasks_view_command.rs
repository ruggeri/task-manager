use actions::ActiveTasksViewAction;
use commands::{FiltererCommand, ScrollCommand, TaskCommand, TaskUpdateCommand, UndoBufferCommand};
use views::ActiveTasksView;
use models::{Direction, End, TaskStatus};

#[derive(Clone, Copy, Debug)]
pub enum ActiveTasksViewCommand {
  Filterer(FiltererCommand),
  Scroll(ScrollCommand),
  Task(TaskCommand),
  UndoBuffer(UndoBufferCommand),
}

impl ActiveTasksViewCommand {
  pub fn from_key(ch: char) -> Option<ActiveTasksView> {
    use self::{
      ActiveTasksViewCommand::*, Direction::*, End::*, FiltererCommand::*, ScrollCommand::*, TaskCommand::*, TaskStatus::*,
      TaskUpdateCommand::*,
    };

    let command = match ch {
      'F' => Filterer(FilterByRequiresInternet),
      '$' => ActiveTasksViewCommand::Scroll(Jump(Bottom)),
      'g' => ActiveTasksViewCommand::Scroll(Jump(Top)),
      '/' => ActiveTasksViewCommand::Scroll(JumpToTask),
      'k' => ActiveTasksViewCommand::Scroll(ScrollCommand::Scroll(Decrease)),
      'j' => ActiveTasksViewCommand::Scroll(ScrollCommand::Scroll(Increase)),
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
      'U' => ActiveTasksViewCommand::Undo(UndoBufferCommand::Redo),
      'u' => ActiveTasksViewCommand::Undo(UndoBufferCommand::Undo),
      _ => return None,
    };

    Some(command)
  }

  pub fn to_action(self, view: &mut ActiveTasksView) -> ActiveTasksViewAction {
    ActiveTasksViewAction::prepare_from_command(self, view)
  }
}
