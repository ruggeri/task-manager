use actions::{Action, TaskAction};
use application::Application;
use models::{Direction, TaskStatus};

#[derive(Clone, Copy, Debug)]
pub enum TaskCommand {
  CreateTask,
  RecordTaskEffort,
  RequestTaskDelay,
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
  pub fn to_action(self, application: &Application) -> Option<Box<dyn Action>> {
    TaskAction::prepare_from_cmd(self, application).map(|ta| {
      // Would be nicer if type ascription were not experimental.
      let ta: Box<dyn Action> = Box::new(ta);
      ta
    })
  }
}
