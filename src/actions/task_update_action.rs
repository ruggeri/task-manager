use commands::TaskUpdateCommand;
use diesel::pg::PgConnection;
use models::*;
use std::rc::Rc;
use util::ui::Window;

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

#[derive(Clone)]
pub enum TaskUpdateAction {
  UpdateDuration {
    update: TaskValueUpdate<TaskDuration>,
    connection: Rc<PgConnection>,
  },
  UpdatePriority {
    update: TaskValueUpdate<TaskPriority>,
    connection: Rc<PgConnection>,
  },
  UpdateRequiresInternet {
    update: TaskValueUpdate<bool>,
    connection: Rc<PgConnection>,
  },
  UpdateStatus {
    update: TaskValueUpdate<TaskStatus>,
    connection: Rc<PgConnection>,
  },
  UpdateTaskTitle {
    update: TaskValueUpdate<String>,
    connection: Rc<PgConnection>,
  },
}

// TODO: Insane level of duplication. Macro time?
impl TaskUpdateAction {
  pub fn prepare_from_cmd(
    cmd: TaskUpdateCommand,
    task: Task,
    window: &Window,
    connection: &Rc<PgConnection>,
  ) -> Option<TaskUpdateAction> {
    use self::TaskUpdateAction as Action;
    use self::TaskUpdateCommand as Cmd;

    match cmd {
      Cmd::EditTaskTitle => {
        let new_task_title = match window.read_line("Edit task title: ") {
          // If they hit Ctrl-C don't make the task afterall.
          None => return None,
          Some(new_task_title) => new_task_title
        };

        TaskValueUpdate::new(task.id, task.title.clone(), new_task_title).map(|update| {
          Action::UpdateTaskTitle {
            update,
            connection: Rc::clone(connection),
          }
        })
      }
      Cmd::ToggleRequiresInternet => {
        TaskValueUpdate::new(task.id, task.requires_internet, !task.requires_internet).map(|update| {
          Action::UpdateRequiresInternet {
            update,
            connection: Rc::clone(connection),
          }
        })
      }
      Cmd::UpdateDuration(direction) => {
        TaskValueUpdate::new(task.id, task.duration, task.duration.increment(direction)).map(|update| {
          Action::UpdateDuration {
            update,
            connection: Rc::clone(connection)
          }
        })
      }
      Cmd::UpdatePriority(direction) => {
        TaskValueUpdate::new(task.id, task.priority, task.priority.increment(direction)).map(|update| {
          Action::UpdatePriority {
            update,
            connection: Rc::clone(connection),
          }
        })
      }
      Cmd::UpdateStatus(new_task_status) => {
        TaskValueUpdate::new(task.id, task.status, new_task_status).map(|update| {
          Action::UpdateStatus {
            update,
            connection: Rc::clone(connection),
          }
        })
      }
    }
  }
}
