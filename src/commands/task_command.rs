use actions::TaskAction;
use diesel::pg::PgConnection;
use models::{Direction, Task, TaskStatus};
use std::rc::Rc;
use util::ui::UserInterface;

#[derive(Clone, Copy, Debug)]
pub enum TaskCommand {
  CreateTask,
  RecordTaskEffort,
  RequestTaskAgeReset,
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
  pub fn to_action<F>(
    self,
    ui: &UserInterface,
    connection: &Rc<PgConnection>,
    current_task_fn: F,
  ) -> Option<TaskAction>
  where
    F: Fn() -> Option<Task>,
  {
    TaskAction::prepare_from_cmd(self, ui, connection, current_task_fn)
  }
}
