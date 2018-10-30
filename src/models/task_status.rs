#[derive(DbEnum, Debug)]
pub enum TaskStatus {
  Abandoned,
  AvailableToPerform,
  Completed,
}
