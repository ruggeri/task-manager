use std::fmt;

#[derive(Clone, Copy, DbEnum, Debug)]
pub enum TaskDuration {
  Long,
  Medium,
  Short,
}

impl fmt::Display for TaskDuration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::TaskDuration::*;

    let s = match self {
      Long => "Long",
      Medium => "Medium",
      Short => "Short",
    };

    f.write_str(s)
  }
}
