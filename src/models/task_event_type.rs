#[derive(Clone, Copy, DbEnum, Debug, Eq, PartialEq)]
pub enum TaskEventType {
  DelayRequested,
  TaskEffortRecorded,
}
