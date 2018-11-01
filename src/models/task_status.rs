use std::fmt;

#[derive(Clone, Copy, DbEnum, Debug, Eq, PartialEq)]
pub enum TaskStatus {
  Abandoned,
  AvailableToPerform,
  Completed,
}

impl fmt::Display for TaskStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self {
      TaskStatus::Abandoned => "Abandoned",
      TaskStatus::AvailableToPerform => "Available",
      TaskStatus::Completed => "Completed",
    };

    f.write_str(s)
  }
}
