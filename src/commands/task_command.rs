use actions::{Action, TaskAction};
use components::Reviewer;
use models::{Direction, TaskStatus};

#[derive(Clone, Copy, Debug)]
pub enum TaskCommand {
  CreateTask,
  RecordTaskEffort,
  UpdateTask(TaskUpdateCommand),
}

#[derive(Clone, Copy, Debug)]
pub enum TaskUpdateCommand {
  EditTaskTitle,
  ToggleRequiresInternet,
  UpdateDuration(Direction),
  UpdatePriority(Direction),
  UpdateStatus(TaskStatus),
}

impl TaskCommand {
  pub fn to_action(self, reviewer: &Reviewer) -> Option<Box<dyn Action>> {
    TaskAction::prepare_from_cmd(self, reviewer).map(|ta| {
      // Would be nicer if type ascription were not experimental.
      let ta: Box<dyn Action> = Box::new(ta);
      ta
    })
  }
}
