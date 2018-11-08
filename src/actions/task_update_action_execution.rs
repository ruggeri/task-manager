use actions::TaskUpdateAction;
use queries::task as task_queries;

macro_rules! update_match {
  ($x:expr, $value:ident, ($( ($enum_value:ident, $update_fn:ident) ),*), ($( ($ref_enum_value:ident, $ref_update_fn:ident) ),*)) => {
    match $x {
      $(TaskUpdateAction::$enum_value {
        task_id,
        connection,
        $value,
        ..
      } => {
        task_queries::$update_fn(*task_id, *$value, connection);
      })*

      $(TaskUpdateAction::$ref_enum_value {
        task_id,
        connection,
        $value,
        ..
      } => {
        task_queries::$ref_update_fn(*task_id, $value, connection);
      })*
    }
  }
}

impl TaskUpdateAction {
  pub fn execute(&mut self) {
    update_match!(
      self,
      new_value,
      (
        (UpdateDuration, update_duration),
        (UpdatePriority, update_priority),
        (UpdateRequiresInternet, update_requires_internet),
        (UpdateStatus, update_status)
      ),
      ((UpdateTaskTitle, update_title))
    )
  }

  pub fn unexecute(&mut self) {
    update_match!(
      self,
      old_value,
      (
        (UpdateDuration, update_duration),
        (UpdatePriority, update_priority),
        (UpdateRequiresInternet, update_requires_internet),
        (UpdateStatus, update_status)
      ),
      ((UpdateTaskTitle, update_title))
    )
  }
}
