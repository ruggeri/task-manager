use super::models::Task;

pub struct Reviewer {
  current_task_idx: usize,
  tasks: Vec<Task>,
}

impl Reviewer {
  pub fn new(tasks: Vec<Task>) -> Reviewer {
    Reviewer { current_task_idx: 0, tasks }
  }

  pub fn print(&self) {
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

      println!("{}", s);

      if idx == self.current_task_idx {
        print!("\x1b[49m")
      }
    }
  }
}
