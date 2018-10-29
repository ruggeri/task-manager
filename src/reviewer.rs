use super::models::Task;

pub struct Reviewer {
  tasks: Vec<Task>,
}

impl Reviewer {
  pub fn new(tasks: Vec<Task>) -> Reviewer {
    Reviewer { tasks }
  }

  pub fn print(&self) {
    for task in &self.tasks {
      let s = format!(
        "{id:4} | {title:50} | {created_at}",
        id = task.id,
        title = task.title,
        created_at = task.created_at
      );
      println!("{}", s);
    }
  }
}
