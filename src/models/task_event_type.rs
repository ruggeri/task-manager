#[derive(Clone, Copy, DbEnum, Debug, Eq, PartialEq)]
pub enum TaskEventType {
  AgeResetRequested,
  DelayRequested,
  TaskEffortRecorded,
}
