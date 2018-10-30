use chrono::{Duration, Utc};
use diesel::pg::PgConnection;
use pancurses;
use ::connection;
use ::models::Task;
use super::commands::{Commands, CommandResult};
use super::scroller::Scroller;

#[repr(C)]
enum ColorPair {
  Default,
  Highlight,
}

pub struct Reviewer {
  window: pancurses::Window,
  pub scroller: Scroller,
  pub connection: PgConnection,
  pub max_tasks: usize,
}

fn initialize_curses() {
  pancurses::start_color();
  pancurses::use_default_colors();
  pancurses::init_pair(ColorPair::Default as i16, -1, -1);
  pancurses::init_pair(ColorPair::Highlight as i16, -1, pancurses::COLOR_BLUE);
  pancurses::noecho();
}

fn format_age(age: Duration) -> String {
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

impl Reviewer {
  pub fn new(max_tasks: usize) -> Reviewer {
    let connection = connection::get();

    let mut reviewer = Reviewer {
      window: pancurses::initscr(),
      scroller: Scroller::new(vec![]),
      connection,
      max_tasks,
    };

    reviewer.refresh();

    reviewer
  }

  pub fn run(&mut self) {
    initialize_curses();

    loop {
      self.display();

      let ch = match self.window.getch().unwrap() {
        pancurses::Input::Character(ch) => ch,
        _ => continue,
      };

      if let CommandResult::ShutDown = Commands::handle_key(self, ch) {
        break;
      }
    }

    pancurses::endwin();
  }

  fn display_header(&self) {
    self.window.attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    self.window.attron(pancurses::A_BOLD);
    self.window.printw(&format!("  {} | {:50} | {:20} | {:12}\n", "id", "title", "last_effort_at", "status"));
    self.window.attroff(pancurses::A_BOLD);
  }

  fn display(&self) {
    self.window.clear();
    self.display_header();

    let iter = self.scroller.tasks.iter().enumerate().take(self.max_tasks);
    for (idx, ref task) in iter {
      self.display_task(idx, task);
    }
  }

  fn display_task(&self, idx: usize, task: &Task) {
    // Choose appropriate color.
    if idx == self.scroller.current_task_idx {
      self.window.attron(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }

    // Display the task line.
    let s = format!(
      "{id:4} | {title:50} | {age:20} | {status:?}\n",
      id = task.id,
      title = task.title,
      age = format_age(task.age_at(Utc::now(), &self.connection)),
      status = task.status,
    );

    // Print the line!
    self.window.printw(&s);

    if idx == self.scroller.current_task_idx {
      self.window.attroff(pancurses::COLOR_PAIR(ColorPair::Highlight as u32));
    }
  }

  pub fn get_new_task_title(&self) -> String {
    self.window.printw("Create new task: ");
    let mut task_title = String::new();

    loop {
      use pancurses::Input::*;

      match self.window.getch().unwrap() {
        Character('\n') => break,
        Character('\x7f') => {
          if task_title.len() == 0 {
            continue;
          }

          task_title.pop();

          let (y, x) = self.window.get_cur_yx();
          self.window.mv(y, x - 1);
          self.window.delch();
        },
        Character(c) => {
          self.window.addch(c);
          task_title.push(c);
        },
        _ => continue,
      }
    }

    task_title
  }

  pub fn refresh(&mut self) {
    let mut tasks = Task::all(&self.connection);

    let current_time = Utc::now();
    tasks.sort_by_key(|t| t.age_at(current_time, &self.connection));
    tasks.reverse();

    self.scroller.refresh(tasks);
  }
}
