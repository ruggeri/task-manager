use models::*;
use components::Reviewer;
use commands::TaskUpdateCommand;

#[derive(Clone, Debug)]
pub struct TaskValueUpdate<T: Eq> {
  pub task_id: i32,
  pub old_value: T,
  pub new_value: T,
}

#[derive(Clone, Debug)]
pub enum TaskUpdateAction {
  UpdateDuration(TaskValueUpdate<TaskDuration>),
  UpdatePriority(TaskValueUpdate<TaskPriority>),
  UpdateRequiresInternet(TaskValueUpdate<bool>),
  UpdateStatus(TaskValueUpdate<TaskStatus>),
  UpdateTaskTitle(TaskValueUpdate<String>),
}

impl TaskUpdateAction {
  pub fn new(cmd: TaskUpdateCommand, task: Task, reviewer: &Reviewer) -> Option<TaskUpdateAction> {
    use self::TaskUpdateCommand as Cmd;
    use self::TaskUpdateAction as Action;

    match cmd {
      Cmd::EditTaskTitle => {
        // TODO: I am not happy that we have to use the reviewer here...
        // Prolly should be a "prepare" method for actions.
        let new_task_title = reviewer.window.read_line("Edit task title: ");
        let tvu = TaskValueUpdate {
          task_id: task.id,
          old_value: task.title.clone(),
          new_value: new_task_title,
        };

        if tvu.old_value == tvu.new_value {
          None
        } else {
          Some(Action::UpdateTaskTitle(tvu))
        }
      }
      Cmd::ToggleRequiresInternet => {
        let tvu = TaskValueUpdate {
          task_id: task.id,
          old_value: task.requires_internet,
          new_value: !task.requires_internet,
        };

        if tvu.old_value == tvu.new_value {
          None
        } else {
          Some(Action::UpdateRequiresInternet(tvu))
        }
      }
      Cmd::UpdateDuration(direction) => {
        let tvu = TaskValueUpdate {
          task_id: task.id,
          old_value: task.duration,
          new_value: task.duration.increment(direction)
        };

        if tvu.old_value == tvu.new_value {
          None
        } else {
          Some(Action::UpdateDuration(tvu))
        }
      }
      Cmd::UpdatePriority(direction) => {
        let tvu = TaskValueUpdate {
          task_id: task.id,
          old_value: task.priority,
          new_value: task.priority.increment(direction)
        };

        if tvu.old_value == tvu.new_value {
          None
        } else {
          Some(Action::UpdatePriority(tvu))
        }
      }
      Cmd::UpdateStatus(new_task_status) => {
        let tvu = TaskValueUpdate {
          task_id: task.id,
          old_value: task.status,
          new_value: new_task_status,
        };

        if tvu.old_value == tvu.new_value {
          None
        } else {
          Some(Action::UpdateStatus(tvu))
        }
      }
    }
  }
}
