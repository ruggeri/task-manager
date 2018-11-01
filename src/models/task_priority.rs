use super::direction::Direction;
use std::fmt;

#[derive(Clone, Copy, DbEnum, Debug, Eq, PartialEq)]
pub enum TaskPriority {
  High,
  Medium,
  Low,
}

impl TaskPriority {
  pub fn increment(&self, direction: Direction) -> TaskPriority {
    use self::Direction::*;
    use self::TaskPriority::*;

    match (direction, *self) {
      (Decrease, Low) => Low,
      (Decrease, Medium) => Low,
      (Decrease, High) => Medium,
      (Increase, Low) => Medium,
      (Increase, Medium) => High,
      (Increase, High) => High,
    }
  }
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
