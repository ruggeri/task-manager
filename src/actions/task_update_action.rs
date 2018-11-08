use commands::TaskUpdateCommand;
use diesel::pg::PgConnection;
use models::*;
use std::rc::Rc;
use util::ui::Window;

#[derive(Clone, Debug)]
pub struct TaskValueUpdate<T: Eq> {
  pub old_value: T,
  pub new_value: T,
}

impl<T: Eq> TaskValueUpdate<T> {
  pub fn new(old_value: T, new_value: T) -> Option<TaskValueUpdate<T>> {
    if old_value == new_value {
      None
    } else {
      Some(TaskValueUpdate {
        old_value,
        new_value,
      })
    }
  }
}

#[derive(Clone)]
pub enum TaskUpdateAction {
  UpdateDuration {
    task_id: i32,
    update: TaskValueUpdate<TaskDuration>,
    connection: Rc<PgConnection>,
  },
  UpdatePriority {
    task_id: i32,
    update: TaskValueUpdate<TaskPriority>,
    connection: Rc<PgConnection>,
  },
  UpdateRequiresInternet {
    task_id: i32,
    update: TaskValueUpdate<bool>,
    connection: Rc<PgConnection>,
  },
  UpdateStatus {
    task_id: i32,
    update: TaskValueUpdate<TaskStatus>,
    connection: Rc<PgConnection>,
  },
  UpdateTaskTitle {
    task_id: i32,
    update: TaskValueUpdate<String>,
    connection: Rc<PgConnection>,
  },
}

// TODO: Insane level of duplication. Macro time?
impl TaskUpdateAction {
  pub fn prepare_from_cmd(
    cmd: TaskUpdateCommand,
    task: &Task,
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
          Some(new_task_title) => new_task_title,
        };

        TaskValueUpdate::new(task.title.clone(), new_task_title).map(|update| {
          Action::UpdateTaskTitle {
            task_id: task.id,
            update,
            connection: Rc::clone(connection),
          }
        })
      }
      Cmd::ToggleRequiresInternet => {
        TaskValueUpdate::new(task.requires_internet, !task.requires_internet).map(|update| {
          Action::UpdateRequiresInternet {
            task_id: task.id,
            update,
            connection: Rc::clone(connection),
          }
        })
      }
      Cmd::UpdateDuration(direction) => {
        TaskValueUpdate::new(task.duration, task.duration.increment(direction)).map(|update| {
          Action::UpdateDuration {
            task_id: task.id,
            update,
            connection: Rc::clone(connection),
          }
        })
      }
      Cmd::UpdatePriority(direction) => {
        TaskValueUpdate::new(task.priority, task.priority.increment(direction)).map(|update| {
          Action::UpdatePriority {
            task_id: task.id,
            update,
            connection: Rc::clone(connection),
          }
        })
      }
      Cmd::UpdateStatus(new_task_status) => {
        TaskValueUpdate::new(task.status, new_task_status).map(|update| Action::UpdateStatus {
          task_id: task.id,
          update,
          connection: Rc::clone(connection),
        })
      }
    }
  }
}
