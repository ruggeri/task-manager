use ::models::{Task, TaskStatus};
use super::reviewer::Reviewer;

use self::Commands::*;
use self::CommandResult::*;

pub enum CommandResult {
  DoNothing,
  ShutDown,
}

#[derive(Debug)]
pub enum Commands {
  Create,
  Destroy,
  RecordTaskEffort,
  ScrollBackward,
  ScrollForward,
  UpdateStatus(TaskStatus),
}

fn create_task(reviewer: &mut Reviewer) {
  let task_title = reviewer.get_new_task_title();
  Task::create(&reviewer.connection, task_title);
  reviewer.refresh();
}

fn destroy(reviewer: &mut Reviewer) {
  match reviewer.scroller.remove_current_task() {
    None => return,
    Some(task) => task.destroy(&reviewer.connection),
  };

  reviewer.refresh();
}

fn record_task_effort(reviewer: &mut Reviewer) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(task) => task.record_effort(&reviewer.connection)
  };

  reviewer.refresh();
}

fn update_status(reviewer: &mut Reviewer, status: TaskStatus) {
  match reviewer.scroller.mut_current_task() {
    None => return,
    Some(task) => task.update_status(status, &reviewer.connection)
  };

  reviewer.refresh();
}

impl Commands {
  pub fn handle_key(reviewer: &mut Reviewer, ch: char) -> CommandResult {
    let command = match ch {
      'j' => ScrollForward,
      'k' => ScrollBackward,
      'a' => UpdateStatus(TaskStatus::Abandoned),
      'c' => UpdateStatus(TaskStatus::Completed),
      'd' => Destroy,
      'n' => Create,
      'q' => return ShutDown,
      'r' => RecordTaskEffort,
      _ => return DoNothing,
    };

    command.execute(reviewer)
  }

  pub fn execute(self, reviewer: &mut Reviewer) -> CommandResult {
    match self {
      Create => create_task(reviewer),
      Destroy => destroy(reviewer),
      RecordTaskEffort => record_task_effort(reviewer),
      ScrollBackward => reviewer.scroller.scroll_backward(),
      ScrollForward => reviewer.scroller.scroll_forward(),
      UpdateStatus(status) => update_status(reviewer, status),
    }

    DoNothing
  }
}
