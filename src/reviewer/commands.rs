use super::reviewer::Reviewer;
use models::{Direction, Task, TaskStatus};

use self::CommandResult::*;
use self::Commands::*;
use self::Direction::*;

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
  EditTaskTitle,
  RecordTaskEffort,
  ScrollBackward,
  ScrollForward,
  JumpToBottom,
  JumpToTask,
  JumpToTop,
  ToggleInternet,
  UpdateDuration(Direction),
  UpdatePriority(Direction),
  UpdateStatus(TaskStatus),
}

fn create_task(reviewer: &Reviewer) {
  let task_title = reviewer.window.read_line("New task title: ");
  Task::create(task_title, &reviewer.connection);
}

fn destroy(reviewer: &Reviewer) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(mut task) => task.destroy(&reviewer.connection),
  };
}

fn edit_task_title(reviewer: &Reviewer) {
  let mut task = match reviewer.scroller.current_task() {
    None => return,
    Some(task) => task,
  };

  let task_title = reviewer.window.read_line("Edit task title: ");
  task.update_title(&task_title, &reviewer.connection);
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

fn update_duration(reviewer: &mut Reviewer, dir: Direction) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(mut task) => task.update_duration(dir, &reviewer.connection),
  };
}

fn update_priority(reviewer: &mut Reviewer, dir: Direction) {
  match reviewer.scroller.current_task() {
    None => return,
    Some(mut task) => task.update_priority(dir, &reviewer.connection),
  };
}

fn jump_to_task(reviewer: &mut Reviewer) {
  let task_id = reviewer.window.read_line("Task id to jump to: ");
  task_id.parse().ok().map(|task_id:i32| {
    reviewer.scroller.jump_to_task_id(task_id)
  });
}

impl Commands {
  pub fn handle_key(reviewer: &mut Reviewer, ch: char) -> CommandResult {
    let command = match ch {
      'j' => ScrollForward,
      'k' => ScrollBackward,
      'i' => ToggleInternet,
      'a' => UpdateStatus(TaskStatus::Abandoned),
      'c' => UpdateStatus(TaskStatus::Completed),
      'e' => EditTaskTitle,
      'n' => Create,
      'q' => return RequestedShutDown,
      'r' => RecordTaskEffort,
      'p' => UpdatePriority(Decrease),
      'P' => UpdatePriority(Increase),
      'd' => UpdateDuration(Decrease),
      'D' => UpdateDuration(Increase),
      '$' => JumpToBottom,
      'g' => JumpToTop,
      '/' => JumpToTask,
      _ => return DidNothing,
    };

    command.execute(reviewer)
  }

  pub fn execute(self, reviewer: &mut Reviewer) -> CommandResult {
    match self {
      Create => {
        create_task(reviewer);
        DidUpdateTaskData
      }
      Destroy => {
        destroy(reviewer);
        DidUpdateTaskData
      }
      EditTaskTitle => {
        edit_task_title(reviewer);
        DidUpdateTaskData
      }
      RecordTaskEffort => {
        record_task_effort(reviewer);
        DidUpdateTaskData
      }
      ScrollBackward => {
        reviewer.scroller.scroll_backward();
        DidUpdateScroller
      }
      ScrollForward => {
        reviewer.scroller.scroll_forward();
        DidUpdateScroller
      }
      ToggleInternet => {
        toggle_internet(reviewer);
        DidUpdateTaskData
      }
      UpdateStatus(status) => {
        update_status(reviewer, status);
        DidUpdateTaskData
      }
      UpdatePriority(dir) => {
        update_priority(reviewer, dir);
        DidUpdateTaskData
      }
      UpdateDuration(dir) => {
        update_duration(reviewer, dir);
        DidUpdateTaskData
      }
      JumpToBottom => {
        reviewer.scroller.jump_to_bottom();
        DidUpdateScroller
      }
      JumpToTask => {
        jump_to_task(reviewer);
        DidUpdateScroller
      }
      JumpToTop => {
        reviewer.scroller.jump_to_top();
        DidUpdateScroller
      }
    }
  }
}
