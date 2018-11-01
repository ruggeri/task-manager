use super::TaskUpdateAction;
use commands::TaskCommand;
use components::Reviewer;
use models::*;

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
  pub fn new(cmd: TaskCommand, reviewer: &Reviewer) -> Option<TaskAction> {
    match cmd {
      TaskCommand::CreateTask => {
        let task_title = reviewer.window.read_line("Edit task title: ");
        Some(TaskAction::CreateTask {
          task_title,
          task: None,
        })
      }
      TaskCommand::RecordTaskEffort => {
        if let Some(task) = reviewer.scroller.current_task() {
          Some(TaskAction::RecordTaskEffort {
            task_id: task.id,
            task_effort: None,
          })
        } else {
          None
        }
      }
      TaskCommand::UpdateTask(cmd) => {
        if let Some(task) = reviewer.scroller.current_task() {
          TaskUpdateAction::new(cmd, task, reviewer).map(|a| TaskAction::TaskUpdate(a))
        } else {
          None
        }
      }
    }
  }
}
