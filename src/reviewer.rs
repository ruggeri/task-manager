use diesel::pg::PgConnection;
use ncurses::*;
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
}

impl Reviewer {
  pub fn new() -> Reviewer {
    let connection = ::establish_connection();
    let tasks = Task::all(&connection);

    Reviewer {
      current_task_idx: 0,
      tasks,
      connection
    }
  }

  pub fn initialize_curses(&self) {
    initscr();
    start_color();
    use_default_colors();
    init_pair(ColorPair::Default as i16, -1, -1);
    init_pair(ColorPair::Highlight as i16, -1, COLOR_BLUE);
  }

  pub fn print(&self) {
    clear();
    attroff(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
    printw("== Tasks ==\n");

    for (idx, ref task) in self.tasks.iter().enumerate() {
      if idx == self.current_task_idx {
        attron(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
      } else {
        attroff(COLOR_PAIR(ColorPair::Highlight as i16) as chtype);
      }

      let s = format!(
        "{id:4} | {title:50} | {created_at}",
        id = task.id,
        title = task.title,
        created_at = task.created_at
      );

      printw(&s);
      printw("\n");
    }
  }

  pub fn scroll_forward(&mut self) {
    if self.current_task_idx < self.tasks.len() - 1 {
      self.current_task_idx += 1
    }
  }

  pub fn scroll_backward(&mut self) {
    if self.current_task_idx > 0 {
      self.current_task_idx -= 1;
    }
  }

  pub fn run(&mut self) {
    self.initialize_curses();

    loop {
      self.print();

      let ch = (getch() as u8) as char;
      self.handle_cmd(ch);
    }
  }

  pub fn destroy(&mut self) {
    self.tasks[self.current_task_idx].destroy(&self.connection);
    self.tasks.remove(self.current_task_idx);
    if self.current_task_idx == self.tasks.len() {
      self.current_task_idx -= 1;
    }
  }

  pub fn handle_cmd(&mut self, ch: char) {
    match ch {
      'j' => self.scroll_forward(),
      'k' => self.scroll_backward(),
      'd' => self.destroy(),
      _ => {},
    }
  }
}
