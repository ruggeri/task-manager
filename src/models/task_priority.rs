use std::fmt;

#[derive(Clone, Copy, DbEnum, Debug)]
pub enum TaskPriority {
  High,
  Medium,
  Low,
}

impl fmt::Display for TaskPriority {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::TaskPriority::*;

    let s = match self {
      High => "High",
      Medium => "Medium",
      Low => "Low",
    };

    f.write_str(s)
  }
}
