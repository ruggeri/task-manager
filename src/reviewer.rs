use chrono::Local;
use diesel::pg::PgConnection;
use ncurses::*;
use self::CommandResult::*;
use super::models::Task;

#[repr(C)]
enum ColorPair {
  Default,
  Highlight,
}

pub struct Reviewer {
  current_task_idx: usize,
  tasks: Vec<Task>,
  connection: PgConnection,
  max_tasks: usize,
}

enum CommandResult {
  DoNothing,
  ShutDown,
}

impl Reviewer {
  pub fn new(max_tasks: usize) -> Reviewer {
    let connection = ::establish_connection();
    let tasks = Task::all(&connection);

    let mut reviewer = Reviewer {
      current_task_idx: 0,
      tasks,
      connection,
      max_tasks,
    };

    reviewer.sort_tasks();

    reviewer
  }

  fn initialize_curses(&self) {
    initscr();
    start_color();
    use_default_colors();
    init_pair(ColorPair::Default as i16, -1, -1);
    init_pair(ColorPair::Highlight as i16, -1, COLOR_BLUE);
    noecho();
  }

  fn print(&self) {
    clear();
    attroff(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
    attr_on(A_BOLD());
    printw(&format!("  {} | {:50} | {:20} | {:12}\n", "id", "title", "last_effort_at", "status"));
    attr_off(A_BOLD());

    for (idx, ref task) in self.tasks.iter().enumerate().take(self.max_tasks) {
      if idx == self.current_task_idx {
        attron(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
      } else {
        attroff(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
      }

      let last_effort_at = match task.last_effort_at(&self.connection) {
        None => String::from("NONE"),
        Some(t) => format!("{}", t.with_timezone(&Local).format("%Y-%m-%d %I:%M %p"))
      };

      let s = format!(
        "{id:4} | {title:50} | {last_effort_at:20} | {status:?}",
        id = task.id,
        title = task.title,
        last_effort_at = last_effort_at,
        status = task.status,
      );

      printw(&s);
      printw("\n");
    }
  }

  fn scroll_forward(&mut self) {
    if self.current_task_idx < self.tasks.len() - 1 {
      self.current_task_idx += 1
    }
  }

  fn scroll_backward(&mut self) {
    if self.current_task_idx > 0 {
      self.current_task_idx -= 1;
    }
  }

  pub fn run(&mut self) {
    self.initialize_curses();

    loop {
      self.print();

      let ch = (getch() as u8) as char;
      if let ShutDown = self.handle_cmd(ch) {
        endwin();
        break;
      }
    }
  }

  fn fix_current_task_idx(&mut self) {
    if self.tasks.len() == 0 {
      self.current_task_idx = 0;
    } else if self.current_task_idx >= self.tasks.len() {
      self.current_task_idx = self.tasks.len() - 1;
    }
  }

  fn abandon(&mut self) {
    self.tasks[self.current_task_idx].abandon(&self.connection);
    self.tasks.remove(self.current_task_idx);
    self.fix_current_task_idx();
  }

  fn complete(&mut self) {
    self.tasks[self.current_task_idx].mark_completed(&self.connection);
    self.tasks.remove(self.current_task_idx);
    self.fix_current_task_idx();
  }

  fn destroy(&mut self) {
    let task = self.tasks.remove(self.current_task_idx);
    task.destroy(&self.connection);
    self.fix_current_task_idx();
  }

  fn create(&mut self) {
    printw("Create new task: ");
    let mut task_title = String::new();

    echo();
    getstr(&mut task_title);
    noecho();

    let t = Task::create(&self.connection, task_title);
    self.tasks.push(t);
    self.sort_tasks();
  }

  fn record_task_effort(&mut self) {
    {
      let current_task = &self.tasks[self.current_task_idx];
      current_task.record_effort(&self.connection);
    }

    self.sort_tasks();
  }

  fn sort_tasks(&mut self) {
    // If I close over self.connection then that's a second borrow?
    let connection = &self.connection;
    self.tasks.sort_by_key(|t|t.sort_time(connection));
  }

  fn handle_cmd(&mut self, ch: char) -> CommandResult {
    match ch {
      'j' => self.scroll_forward(),
      'k' => self.scroll_backward(),
      'a' => {
        self.abandon();
      },
      'c' => {
        self.complete();
      },
      'd' => {
        self.destroy();
      },
      'n' => {
        self.create();
      },
      'q' => return ShutDown,
      'r' => self.record_task_effort(),
      _ => {},
    }

    DoNothing
  }
}
