use super::TaskUpdateAction;
use commands::TaskCommand;
use components::Reviewer;
use models::{Task, TaskEffort};

#[derive(Clone, Debug)]
pub enum TaskAction {
  CreateTask {
    task_title: String,
    task: Option<Task>,
  },
  RecordTaskEffort {
    task_id: i32,
    task_effort: Option<TaskEffort>,
  },
  TaskUpdate(TaskUpdateAction),
}

impl TaskAction {
  pub fn from_cmd(cmd: TaskCommand, reviewer: &Reviewer) -> Option<TaskAction> {
    match cmd {
      TaskCommand::CreateTask => {
        let task_title = reviewer.window.read_line("Edit task title: ");
        Some(TaskAction::CreateTask {
          task_title,
          task: None,
        })
      }
      TaskCommand::RecordTaskEffort => {
        reviewer.scroller.current_task().and_then(|task| {
          Some(TaskAction::RecordTaskEffort {
            task_id: task.id,
            task_effort: None,
          })
        })
      }
      TaskCommand::UpdateTask(cmd) => {
        reviewer.scroller.current_task().and_then(|task| {
          TaskUpdateAction::from_cmd(cmd, &task, reviewer)
        }).map(|ta| {
          TaskAction::TaskUpdate(ta)
        })
      }
    }
  }
}
