use std::cell::Cell;
use std::io::stdin;
use super::models::Task;

pub struct Reviewer {
  current_task_idx: Cell<usize>,
  tasks: Vec<Task>,
}

impl Reviewer {
  pub fn new(tasks: Vec<Task>) -> Reviewer {
    Reviewer { current_task_idx: Cell::new(0), tasks }
  }

  pub fn print(&self) {
    print!("\x1b[H[J");
    println!("== Tasks ==");

    let current_task_idx = self.current_task_idx.get();
    for (idx, ref task) in self.tasks.iter().enumerate() {
      if idx == current_task_idx {
        print!("\x1b[42m")
      }

      let s = format!(
        "{id:4} | {title:50} | {created_at}",
        id = task.id,
        title = task.title,
        created_at = task.created_at
      );

      print!("{}", s);

      if idx == current_task_idx {
        print!("\x1b[49m")
      }

      println!("");
    }
  }

  pub fn scroll_forward(&self) {
    let current_task_idx = self.current_task_idx.get();
    if current_task_idx < self.tasks.len() - 1 {
      self.current_task_idx.set(current_task_idx + 1);
    }
  }

  pub fn scroll_backward(&self) {
    let current_task_idx = self.current_task_idx.get();
    if current_task_idx > 0 {
      self.current_task_idx.set(current_task_idx - 1);
    }
  }

  pub fn run(&self) {
    loop {
      self.print();

      let mut cmd = String::new();
      stdin().read_line(&mut cmd).unwrap();
      // remove newline
      cmd.pop();

      self.handle_cmd(&cmd);
    }
  }

  pub fn handle_cmd(&self, cmd: &str) {
    match cmd {
      "next" => self.scroll_forward(),
      "prev" => self.scroll_backward(),
      _ => {},
    }
  }
}
