use super::Direction;

#[derive(Clone, Copy, DbEnum, Debug, Eq, PartialEq)]
pub enum TaskDuration {
  Long,
  Medium,
  Short,
}

impl TaskDuration {
  pub fn increment(self, direction: Direction) -> TaskDuration {
    use self::Direction::*;
    use self::TaskDuration::*;

    match (direction, self) {
      (Decrease, Short) => Short,
      (Decrease, Medium) => Short,
      (Decrease, Long) => Medium,
      (Increase, Short) => Medium,
      (Increase, Medium) => Long,
      (Increase, Long) => Long,
    }
  }
}
