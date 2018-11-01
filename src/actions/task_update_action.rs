use commands::TaskUpdateCommand;
use components::Reviewer;
use models::*;

#[derive(Clone, Debug)]
pub struct TaskValueUpdate<T: Eq> {
  pub task_id: i32,
  pub old_value: T,
  pub new_value: T,
}

impl<T: Eq> TaskValueUpdate<T> {
  pub fn new(task_id: i32, old_value: T, new_value: T) -> Option<TaskValueUpdate<T>> {
    if old_value == new_value {
      None
    } else {
      Some(TaskValueUpdate { task_id, old_value, new_value })
    }
  }
}

#[derive(Clone, Debug)]
pub enum TaskUpdateAction {
  UpdateDuration(TaskValueUpdate<TaskDuration>),
  UpdatePriority(TaskValueUpdate<TaskPriority>),
  UpdateRequiresInternet(TaskValueUpdate<bool>),
  UpdateStatus(TaskValueUpdate<TaskStatus>),
  UpdateTaskTitle(TaskValueUpdate<String>),
}

// TODO: Insane level of duplication. Macro time?
impl TaskUpdateAction {
  pub fn prepare_from_cmd(
    cmd: TaskUpdateCommand,
    task: &Task,
    reviewer: &Reviewer,
  ) -> Option<TaskUpdateAction> {
    use self::TaskUpdateAction as Action;
    use self::TaskUpdateCommand as Cmd;

    match cmd {
      Cmd::EditTaskTitle => {
        let new_task_title = reviewer.window.read_line("Edit task title: ");
        TaskValueUpdate::new(task.id, task.title.clone(), new_task_title).map(|tvu| {
          Action::UpdateTaskTitle(tvu)
        })
      }
      Cmd::ToggleRequiresInternet => {
        TaskValueUpdate::new(task.id, task.requires_internet, !task.requires_internet).map(|tvu| {
          Action::UpdateRequiresInternet(tvu)
        })
      }
      Cmd::UpdateDuration(direction) => {
        TaskValueUpdate::new(task.id, task.duration, task.duration.increment(direction)).map(|tvu| {
          Action::UpdateDuration(tvu)
        })
      }
      Cmd::UpdatePriority(direction) => {
        TaskValueUpdate::new(task.id, task.priority, task.priority.increment(direction)).map(|tvu| {
          Action::UpdatePriority(tvu)
        })
      }
      Cmd::UpdateStatus(new_task_status) => {
        TaskValueUpdate::new(task.id, task.status, new_task_status).map(|tvu| {
          Action::UpdateStatus(tvu)
        })
      }
    }
  }
}
