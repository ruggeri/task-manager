use super::TaskUpdateAction;
use commands::TaskCommand;
use components::Reviewer;
use models::{Task, TaskEvent};

#[derive(Clone, Debug)]
pub enum TaskAction {
  CreateTask {
    task_title: String,
    task: Option<Task>,
  },
  RecordTaskEffort {
    task_id: i32,
    task_event: Option<TaskEvent>,
  },
  TaskUpdate(TaskUpdateAction),
}

impl TaskAction {
  pub fn prepare_from_cmd(cmd: TaskCommand, reviewer: &Reviewer) -> Option<TaskAction> {
    match cmd {
      // Create a task.
      TaskCommand::CreateTask => {
        let task_title = match reviewer.window.read_line("Edit task title: ") {
          // If they hit Ctrl-C don't make the task afterall.
          None => return None,
          Some(task_title) => task_title
        };

        Some(TaskAction::CreateTask {
          task_title,
          task: None,
        })
      }

      // Record a task effort.
      TaskCommand::RecordTaskEffort => reviewer.scroller.current_task().and_then(|task| {
        Some(TaskAction::RecordTaskEffort {
          task_id: task.id,
          task_event: None,
        })
      }),

      // Update a task attribute.
      TaskCommand::UpdateTask(cmd) => reviewer
        .scroller
        .current_task()
        .and_then(|task| TaskUpdateAction::prepare_from_cmd(cmd, &task, reviewer))
        .map(|ta| TaskAction::TaskUpdate(ta)),
    }
  }
}
