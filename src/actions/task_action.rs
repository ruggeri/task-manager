use actions::TaskUpdateAction;
use commands::TaskCommand;
use diesel::pg::PgConnection;
use models::{Task, TaskEvent};
use std::rc::Rc;
use util::ui::Window;

#[derive(Clone)]
pub enum TaskAction {
  CreateTask {
    task_title: String,
    task: Option<Task>,
    connection: Rc<PgConnection>,
  },
  RecordTaskEffort {
    task_id: i32,
    task_event: Option<TaskEvent>,
    connection: Rc<PgConnection>,
  },
  RequestTaskDelay {
    task_id: i32,
    task_event: Option<TaskEvent>,
    connection: Rc<PgConnection>,
  },
  TaskUpdate(TaskUpdateAction),
}

impl TaskAction {
  pub fn prepare_from_cmd<F>(cmd: TaskCommand, window: &Window, connection: &Rc<PgConnection>, current_task_fn: F) -> Option<TaskAction>
  where F: Fn() -> Option<Task> {
    match cmd {
      // Create a task.
      TaskCommand::CreateTask => {
        let task_title = match window.read_line("Edit task title: ") {
          // If they hit Ctrl-C don't make the task afterall.
          None => return None,
          Some(task_title) => task_title
        };

        Some(TaskAction::CreateTask {
          task_title,
          task: None,
          connection: Rc::clone(connection),
        })
      }

      // Record a task effort.
      TaskCommand::RecordTaskEffort => current_task_fn().and_then(|task| {
        Some(TaskAction::RecordTaskEffort {
          task_id: task.id,
          task_event: None,
          connection: Rc::clone(connection),
        })
      }),

      // Request a task delay.
      TaskCommand::RequestTaskDelay => current_task_fn().and_then(|task| {
        Some(TaskAction::RequestTaskDelay {
          task_id: task.id,
          task_event: None,
          connection: Rc::clone(connection),
        })
      }),

      // Update a task attribute.
      TaskCommand::UpdateTask(cmd) => current_task_fn()
        .and_then(|task| TaskUpdateAction::prepare_from_cmd(cmd, task, window, connection))
        .map(|ta| TaskAction::TaskUpdate(ta)),
    }
  }
}
