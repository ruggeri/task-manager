use super::direction::Direction;
use std::fmt;

#[derive(Clone, Copy, DbEnum, Debug, Eq, PartialEq)]
pub enum TaskDuration {
  Long,
  Medium,
  Short,
}

impl TaskDuration {
  pub fn increment(&self, direction: Direction) -> TaskDuration {
    use self::Direction::*;
    use self::TaskDuration::*;

    match (direction, *self) {
      (Decrease, Short) => Short,
      (Decrease, Medium) => Short,
      (Decrease, Long) => Medium,
      (Increase, Short) => Medium,
      (Increase, Medium) => Long,
      (Increase, Long) => Long,
    }
  }
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
