use models::Task;

pub struct Scroller {
  pub current_task_idx: usize,
  pub tasks: Vec<Task>,
}

impl Scroller {
  pub fn new(tasks: Vec<Task>) -> Scroller {
    Scroller {
      current_task_idx: 0,
      tasks,
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

  pub fn current_task(&self) -> Option<&Task> {
    if self.tasks.len() == 0 {
      None
    } else {
      Some(&self.tasks[self.current_task_idx])
    }
  }

  pub fn mut_current_task(&mut self) -> Option<&mut Task> {
    if self.tasks.len() == 0 {
      None
    } else {
      Some(&mut self.tasks[self.current_task_idx])
    }
  }

  pub fn remove_current_task(&mut self) -> Option<Task> {
    if self.tasks.len() == 0 {
      None
    } else {
      Some(self.tasks.remove(self.current_task_idx))
    }
  }

  pub fn refresh(&mut self, tasks: Vec<Task>) {
    self.tasks = tasks;

    let num_tasks = self.tasks.len();

    if num_tasks == 0 {
      self.current_task_idx = 0;
    } else if self.current_task_idx >= num_tasks {
      self.current_task_idx = num_tasks - 1;
    }
  }
}
