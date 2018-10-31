use super::data_source::TaskResult;
use super::scroller::Scroller;
use super::ui;
use super::ui::ColorPair;
use chrono::Duration;
use pancurses;
use std::rc::Rc;

fn format_task_age(age: Duration) -> String {
  let weeks = age.num_weeks();
  let days = age.num_days() - 7 * age.num_weeks();
  let hours = age.num_hours() - 24 * age.num_days();
  let mins = age.num_minutes() - 60 * age.num_hours();

  if weeks > 0 {
    format!("{} weeks {} days", weeks, days)
  } else if days > 0 {
    format!("{} days {} hours", days, hours)
  } else if hours > 0 {
    format!("{} hours {} mins", hours, mins)
  } else if mins > 0 {
    format!("{} mins", mins)
  } else {
    String::from("now")
  }
}

pub struct TaskResultsWindow {
  window: Rc<ui::Window>,
  scroller: Rc<Scroller>,
}

impl TaskResultsWindow {
  pub fn new(window: &Rc<ui::Window>, scroller: &Rc<Scroller>) -> TaskResultsWindow {
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
    self.display_header();

    // TODO: This is a bad idea. I want to have interior mutability, but
    // here we can see it's sneaking out.
    for (idx, ref result) in self.scroller.results().iter().enumerate() {
      self.display_result(idx, result);
    }
  }

  fn display_header(&self) {
    let pwindow = self.pwindow();
    pwindow.attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    pwindow.attron(pancurses::A_BOLD);
    pwindow.printw(&format!(
      "  {} | {:50} | {:20} | {:10} | {:12} | {:8} | {:9} \n",
      "id", "title", "last_effort_at", "status", "internet", "priority", "duration"
    ));
    pwindow.attroff(pancurses::A_BOLD);
  }

  fn display_result(&self, idx: usize, result: &TaskResult) {
    let pwindow = self.pwindow();

    // Choose appropriate color.
    if idx == self.scroller.current_result_idx() {
      pwindow.attron(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }

    let requires_internet = if result.task.requires_internet {
      "Yes Internet"
    } else {
      "No Internet"
    };

    // Display the task line.
    let s = format!(
      "{id:4} | {title:50} | {age:20} | {status:10} | {requires_internet:12} | {priority:8} | {duration:9} \n",
      id = result.task.id,
      title = result.task.title,
      age = format_task_age(result.task_age),
      status = format!("{}", result.task.status),
      requires_internet = requires_internet,
      priority = format!("{}", result.task.priority),
      duration = format!("{}", result.task.duration),
    );

    // Print the line!
    pwindow.printw(&s);

    if idx == self.scroller.current_result_idx() {
      pwindow.attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }
  }
}
