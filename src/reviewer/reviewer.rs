use chrono::Local;
use diesel::pg::PgConnection;
use ncurses::*;
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
  pub scroller: Scroller,
  pub connection: PgConnection,
  pub max_tasks: usize,
}

fn initialize_curses() {
  initscr();
  start_color();
  use_default_colors();
  init_pair(ColorPair::Default as i16, -1, -1);
  init_pair(ColorPair::Highlight as i16, -1, COLOR_BLUE);
  noecho();
}

impl Reviewer {
  pub fn new(max_tasks: usize) -> Reviewer {
    let connection = connection::get();

    let mut reviewer = Reviewer {
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

      let ch = (getch() as u8) as char;
      if let CommandResult::ShutDown = Commands::handle_key(self, ch) {
        endwin();
        break;
      }
    }
  }

  fn display_header(&self) {
    attroff(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
    attr_on(A_BOLD());
    printw(&format!("  {} | {:50} | {:20} | {:12}\n", "id", "title", "last_effort_at", "status"));
    attr_off(A_BOLD());
  }

  fn display(&self) {
    clear();
    self.display_header();

    let iter = self.scroller.tasks.iter().enumerate().take(self.max_tasks);
    for (idx, ref task) in iter {
      self.display_task(idx, task);
    }
  }

  fn display_task(&self, idx: usize, task: &Task) {
    // Choose appropriate color.
    if idx == self.scroller.current_task_idx {
      attron(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
    } else {
      attroff(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
    }

    // Format time zone.
    let last_effort_at = match task.last_effort_at(&self.connection) {
      None => String::from("NONE"),
      Some(t) => format!("{}", t.with_timezone(&Local).format("%Y-%m-%d %I:%M %p"))
    };

    // Display the task line.
    let s = format!(
      "{id:4} | {title:50} | {last_effort_at:20} | {status:?}\n",
      id = task.id,
      title = task.title,
      last_effort_at = last_effort_at,
      status = task.status,
    );

    // Print the line!
    printw(&s);
  }

  pub fn get_new_task_title(&self) -> String {
    printw("Create new task: ");
    let mut task_title = String::new();

    echo();
    getstr(&mut task_title);
    noecho();

    task_title
  }

  pub fn refresh(&mut self) {
    let mut tasks = Task::all(&self.connection);
    tasks.sort_by_key(|t| t.sort_time(&self.connection));
    self.scroller.refresh(tasks);
  }
}
