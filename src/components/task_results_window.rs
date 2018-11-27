use chrono::Duration;
use components::{
  result::TaskResult,
  scrollers::{ScrollerEvent, ScrollerState},
};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use util::{
  line_buffer::{Line, LineBuffer},
  ui::{ColorPair, UserInterface}
};

type ResultsVec = Rc<Vec<TaskResult>>;

fn format_task_age(age: Duration) -> String {
  let weeks = age.num_weeks();
  let days = age.num_days() - 7 * age.num_weeks();
  let hours = age.num_hours() - 24 * age.num_days();
  let mins = age.num_minutes() - 60 * age.num_hours();

  if weeks > 0 {
    format!("{}w {}d", weeks, days)
  } else if days > 0 {
    format!("{}d {}h", days, hours)
  } else if hours > 0 {
    format!("{}h {}m", hours, mins)
  } else if mins > 0 {
    format!("{}m", mins)
  } else {
    String::from("now")
  }
}

pub struct TaskResultsWindow {
  line_buffer: LineBuffer,
  scroller_state: RefCell<Option<ScrollerState<TaskResult>>>,
}

// TODO: Can I clean this code up at all?
impl TaskResultsWindow {
  pub fn new(ui: &Rc<UserInterface>) -> TaskResultsWindow {
    TaskResultsWindow {
      line_buffer: LineBuffer::new(ui),
      scroller_state: RefCell::new(None),
    }
  }

  fn results(&self) -> ResultsVec {
    self
      .scroller_state
      .borrow()
      .as_ref()
      .expect(
        "scroller_state should be set before trying to use results",
      ).results
      .clone()
  }

  fn current_result_idx(&self) -> usize {
    self
      .scroller_state
      .borrow()
      .as_ref()
      .expect(
        "scroller_state should be set before trying to use result idx",
      ).current_result_idx as usize
  }

  fn save_scroller_state(&self, state: ScrollerState<TaskResult>) {
    *self.scroller_state.borrow_mut() = Some(state);
  }

  fn save_current_result_idx(&self, current_result_idx: i32) {
    let mut state = self.scroller_state.borrow_mut();
    let state = state.deref_mut();
    match state {
      None => panic!("scroller_state should be set before trying to set result idx individually"),
      Some(state) => state.current_result_idx = current_result_idx,
    }
  }

  pub fn redraw(&self, event: ScrollerEvent<TaskResult>) {
    match event {
      ScrollerEvent::ChangedScrollPosition {
        old_result_idx,
        new_state:
          ScrollerState {
            current_result_idx, ..
          },
      } => {
        self.save_current_result_idx(current_result_idx);
        self.incremental_redraw(old_result_idx as usize, current_result_idx as usize);
      }
      ScrollerEvent::GotNewScrollResults { state } => {
        self.save_scroller_state(state);
        self.full_redraw();
      }
    }

    self.line_buffer.redraw();
  }

  pub fn full_redraw(&self) {
    self.display_header();
    for (idx, result) in self.results().iter().enumerate() {
      self.display_result(idx, result);
    }
  }

  pub fn max_title_len(&self) -> usize {
    self
      .results()
      .iter()
      .map(|r| r.task.title.len())
      .max()
      .unwrap_or(0)
  }

  pub fn incremental_redraw(
    &self,
    old_result_idx: usize,
    current_result_idx: usize,
  ) {
    let results = &self.results();
    self.display_result(
      old_result_idx,
      &results[old_result_idx],
    );
    self.display_result(
      current_result_idx,
      &results[current_result_idx],
    );
  }

  fn display_header(&self) {
    let text = format!(
      " {title:title_width$} | {priority:5} | {durration:5} | {age:8} | {status:6} | {requires_internet:6} \n",
      title = "title",
      title_width = ::std::cmp::max(5, self.max_title_len() + 2),
      priority = "prior",
      durration = "durr",
      age = "age",
      status = "stat",
      requires_internet = "net",
    );

    self.line_buffer.set_line(0, Line { text, color: ColorPair::Bold });
  }

  fn display_result(&self, idx: usize, result: &TaskResult) {
    // Choose appropriate color.
    let color = if idx == self.current_result_idx() {
      ColorPair::Highlight
    } else {
      ColorPair::Default
    };

    let priority = {
      use models::TaskPriority::*;
      match result.task.priority {
        Low => "Low",
        Medium => "Med",
        High => "High",
      }
    };

    let duration = {
      use models::TaskDuration::*;
      match result.task.duration {
        Short => "Short",
        Medium => "Med",
        Long => "Long",
      }
    };

    let status = {
      use models::TaskStatus::*;
      match result.task.status {
        Abandoned => "Aband",
        AvailableToPerform => "Avail",
        Completed => "Compl",
      }
    };

    let requires_internet = if result.task.requires_internet {
      "+net"
    } else {
      "-net"
    };

    // Display the task line.
    let text = format!(
      " {title:title_width$} | {priority:5} | {duration:5} | {age:8} | {status:6} | {requires_internet:6}\n",
      title = result.task.title,
      title_width = ::std::cmp::max(5, self.max_title_len() + 2),
      priority = priority,
      duration = duration,
      age = format_task_age(result.last_effort_duration_since),
      status = status,
      requires_internet = requires_internet,
    );

    // Print the line!
    self.line_buffer.set_line(idx + 1, Line { text, color });
  }
}
