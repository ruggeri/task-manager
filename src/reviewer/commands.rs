use super::reviewer::Reviewer;
use models::{Task, TaskStatus};

use self::CommandResult::*;
use self::Commands::*;

pub enum CommandResult {
  DidNothing,
  DidUpdateScroller,
  DidUpdateTaskData,
  RequestedShutDown,
}

#[derive(Debug)]
pub enum Commands {
  Create,
  Destroy,
  RecordTaskEffort,
  ScrollBackward,
  ScrollForward,
  ToggleInternet,
  UpdateStatus(TaskStatus),
}

fn create_task(reviewer: &Reviewer) {
  let task_title = reviewer.window.read_line();
  Task::create(&reviewer.connection, task_title);
}

fn destroy(reviewer: &Reviewer) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(mut task) => task.destroy(&reviewer.connection),
  };
}

fn record_task_effort(reviewer: &Reviewer) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(task) => task.record_effort(&reviewer.connection),
  };
}

fn toggle_internet(reviewer: &mut Reviewer) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(mut task) => task.toggle_internet(&reviewer.connection),
  };
}

fn update_status(reviewer: &mut Reviewer, status: TaskStatus) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(mut task) => task.update_status(status, &reviewer.connection),
  };
}

impl Commands {
  pub fn handle_key(reviewer: &mut Reviewer, ch: char) -> CommandResult {
    let command = match ch {
      'j' => ScrollForward,
      'k' => ScrollBackward,
      'i' => ToggleInternet,
      'a' => UpdateStatus(TaskStatus::Abandoned),
      'c' => UpdateStatus(TaskStatus::Completed),
      'd' => Destroy,
      'n' => Create,
      'q' => return RequestedShutDown,
      'r' => RecordTaskEffort,
      _ => return DidNothing,
    };

    command.execute(reviewer)
  }

  pub fn execute(self, reviewer: &mut Reviewer) -> CommandResult {
    match self {
      Create => {
        create_task(reviewer);
        DidUpdateTaskData
      },
      Destroy => {
        destroy(reviewer);
        DidUpdateTaskData
      },
      RecordTaskEffort => {
        record_task_effort(reviewer);
        DidUpdateTaskData
      },
      ScrollBackward => {
        reviewer.scroller.scroll_backward();
        DidUpdateScroller
      },
      ScrollForward => {
        reviewer.scroller.scroll_forward();
        DidUpdateScroller
      },
      ToggleInternet => {
        toggle_internet(reviewer);
        DidUpdateTaskData
      },
      UpdateStatus(status) => {
        update_status(reviewer, status);
        DidUpdateTaskData
      },
    }
  }
}
