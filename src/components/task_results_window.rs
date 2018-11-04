use super::data_source;
use super::Scroller;
use chrono::Duration;
use pancurses;
use std::rc::Rc;
use util::ui::{ColorPair, Window as UiWindow};

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
  window: Rc<UiWindow>,
  scroller: Rc<Scroller>,
}

impl TaskResultsWindow {
  pub fn new(window: &Rc<UiWindow>, scroller: &Rc<Scroller>) -> TaskResultsWindow {
    TaskResultsWindow {
      window: Rc::clone(window),
      scroller: Rc::clone(scroller),
    }
  }

  fn pwindow(&self) -> &pancurses::Window {
    &self.window.window
  }

  pub fn redraw(&self) {
    self.pwindow().clear();

    let max_title_len = self
      .scroller
      .results()
      .iter()
      .map(|r| r.task.title.len())
      .max()
      .unwrap_or(0);

    self.display_header(max_title_len);

    // TODO: This is a bad idea. I want to have interior mutability, but
    // here we can see it's sneaking out.
    for (idx, ref result) in self.scroller.results().iter().enumerate() {
      self.display_result(idx as i32, result, max_title_len);
    }
  }

  fn display_header(&self, max_title_len: usize) {
    let pwindow = self.pwindow();
    pwindow.attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    pwindow.attron(pancurses::A_BOLD);
    pwindow.printw(&format!(
      "  {} | {:title_width$} | {:5} | {:4} | {:8} | {:6} | {:6} \n",
      "id", "title", "prior", "durr", "age", "stat", "net",
      title_width = ::std::cmp::max(5, max_title_len + 2),
    ));
    pwindow.attroff(pancurses::A_BOLD);
  }

  fn display_result(&self, idx: i32, result: &data_source::Result, max_title_len: usize) {
    let pwindow = self.pwindow();

    // Choose appropriate color.
    if idx == self.scroller.current_result_idx() {
      pwindow.attron(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }

    let priority = {
      use models::TaskPriority::*;
      match result.task.priority {
        Low => "Lo", Medium => "Me", High => "Hi",
      }
    };

    let duration = {
      use models::TaskDuration::*;
      match result.task.duration {
        Short => "Sh", Medium => "Me", Long => "Lo",
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
      "{id:4} | {title:title_width$} | {priority:5} | {duration:4} | {age:8} | {status:6} | {requires_internet:6}\n",
      id = result.task.id,
      title = result.task.title,
      title_width = ::std::cmp::max(5, max_title_len + 2),
      priority = priority,
      duration = duration,
      age = format_task_age(result.last_effort_duration_since),
      status = status,
      requires_internet = requires_internet,
    );

    // Print the line!
    pwindow.printw(&s);

    if idx == self.scroller.current_result_idx() {
      pwindow.attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }
  }
}
