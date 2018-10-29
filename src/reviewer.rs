use diesel::pg::PgConnection;
use std::io::stdin;
use super::models::Task;

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

  pub fn print(&self) {
    print!("\x1b[H[J");
    println!("== Tasks ==");

    for (idx, ref task) in self.tasks.iter().enumerate() {
      if idx == self.current_task_idx {
        print!("\x1b[42m")
      }

      let s = format!(
        "{id:4} | {title:50} | {created_at}",
        id = task.id,
        title = task.title,
        created_at = task.created_at
      );

      print!("{}", s);

      if idx == self.current_task_idx {
        print!("\x1b[49m")
      }

      println!("");
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
    loop {
      self.print();

      let mut cmd = String::new();
      stdin().read_line(&mut cmd).unwrap();
      // remove newline
      cmd.pop();

      self.handle_cmd(&cmd);
    }
  }

  pub fn destroy(&mut self) {
    self.tasks[self.current_task_idx].destroy(&self.connection);
    self.tasks.remove(self.current_task_idx);
    if self.current_task_idx == self.tasks.len() {
      self.current_task_idx -= 1;
    }
  }

  pub fn handle_cmd(&mut self, cmd: &str) {
    match cmd {
      "next" => self.scroll_forward(),
      "prev" => self.scroll_backward(),
      "destroy" => self.destroy(),
      _ => {},
    }
  }
}
