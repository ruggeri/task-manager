#[derive(Clone, Copy, DbEnum, Debug, Eq, PartialEq)]
pub enum TaskStatus {
  Abandoned,
  AvailableToPerform,
  Completed,
}
