use chrono::Duration;
use components::{
  data_source,
  scroller::{ScrollerEvent, ScrollerState},
};
use pancurses;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use util::ui::{ColorPair, UserInterface};

type ResultsVec = Rc<Vec<data_source::Result>>;

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
  ui: Rc<UserInterface>,
  scroller_state: RefCell<Option<ScrollerState>>,
}

// TODO: Can I clean this code up at all?
impl TaskResultsWindow {
  pub fn new(ui: &Rc<UserInterface>) -> TaskResultsWindow {
    TaskResultsWindow {
      ui: Rc::clone(ui),
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

  fn current_result_idx(&self) -> i32 {
    self
      .scroller_state
      .borrow()
      .as_ref()
      .expect(
        "scroller_state should be set before trying to use result idx",
      ).current_result_idx
  }

  fn save_scroller_state(&self, state: ScrollerState) {
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

  fn pwindow(&self) -> &pancurses::Window {
    &self.ui.window
  }

  pub fn redraw(&self, event: ScrollerEvent) {
    match event {
      ScrollerEvent::ChangedScrollPosition {
        old_result_idx,
        new_state:
          ScrollerState {
            current_result_idx, ..
          },
      } => {
        self.save_current_result_idx(current_result_idx);
        self.incremental_redraw(old_result_idx, current_result_idx);
        // Position cursor at bottom for text input.
        self.pwindow().mv((self.results().len() + 1) as i32, 0);
      }
      ScrollerEvent::GotNewScrollResults { state } => {
        self.save_scroller_state(state);
        self.full_redraw();
      }
    }
  }

  pub fn full_redraw(&self) {
    self.pwindow().clear();

    self.display_header();
    for (idx, result) in self.results().iter().enumerate() {
      self.display_result(idx as i32, result);
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
    old_result_idx: i32,
    current_result_idx: i32,
  ) {
    let results = &self.results();
    self.display_result(
      old_result_idx,
      &results[old_result_idx as usize],
    );
    self.display_result(
      current_result_idx,
      &results[current_result_idx as usize],
    );
  }

  fn display_header(&self) {
    let pwindow = self.pwindow();
    pwindow.attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    pwindow.attron(pancurses::A_BOLD);
    pwindow.printw(&format!(
      "  {id} | {title:title_width$} | {priority:5} | {durration:5} | {age:8} | {status:6} | {requires_internet:6} \n",
      id = "id",
      title = "title",
      title_width = ::std::cmp::max(5, self.max_title_len() + 2),
      priority = "prior",
      durration = "durr",
      age = "age",
      status = "stat",
      requires_internet = "net",
    ));
    pwindow.attroff(pancurses::A_BOLD);
  }

  fn display_result(&self, idx: i32, result: &data_source::Result) {
    let pwindow = self.pwindow();
    pwindow.mv(idx + 1, 0);

    // Choose appropriate color.
    if idx == self.current_result_idx() {
      pwindow
        .attron(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }

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
    let s = format!(
      "{id:4} | {title:title_width$} | {priority:5} | {duration:5} | {age:8} | {status:6} | {requires_internet:6}\n",
      id = result.task.id,
      title = result.task.title,
      title_width = ::std::cmp::max(5, self.max_title_len() + 2),
      priority = priority,
      duration = duration,
      age = format_task_age(result.last_effort_duration_since),
      status = status,
      requires_internet = requires_internet,
    );

    // Print the line!
    pwindow.printw(&s);

    if idx == self.current_result_idx() {
      pwindow
        .attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }
  }
}
