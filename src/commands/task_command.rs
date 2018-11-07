use actions::TaskAction;
use diesel::pg::PgConnection;
use models::{Direction, TaskStatus};
use std::rc::Rc;
use util::ui::Window;

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
  pub fn to_action<F>(self, window: &Window, connection: &Rc<PgConnection>, currentTaskFn: F) -> Option<TaskAction> {
    TaskAction::prepare_from_cmd(self, window, connection, currentTaskFn)
  }
}
