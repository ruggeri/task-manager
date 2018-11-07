use actions::TaskUpdateAction;
use queries::task as task_queries;

// TODO: Insane level of duplication. Macro time?
impl TaskUpdateAction {
  pub fn execute(&mut self) {
    use self::TaskUpdateAction::*;
    match self {
      UpdateDuration { update, connection } => {
        task_queries::update_duration(update.task_id, update.new_value, connection);
      }
      UpdatePriority { update, connection } => {
        task_queries::update_priority(update.task_id, update.new_value, connection);
      }
      UpdateRequiresInternet { update, connection } => {
        task_queries::update_requires_internet(update.task_id, update.new_value, connection);
      }
      UpdateStatus { update, connection } => {
        task_queries::update_status(update.task_id, update.new_value, connection);
      }
      UpdateTaskTitle { update, connection } => {
        task_queries::update_title(update.task_id, &update.new_value, connection);
      }
    }
  }

  pub fn unexecute(&mut self) {
    use self::TaskUpdateAction::*;
    match self {
      UpdateDuration { update, connection } => {
        task_queries::update_duration(update.task_id, update.old_value, connection);
      }
      UpdatePriority { update, connection } => {
        task_queries::update_priority(update.task_id, update.old_value, connection);
      }
      UpdateRequiresInternet { update, connection } => {
        task_queries::update_requires_internet(update.task_id, update.old_value, connection);
      }
      UpdateStatus { update, connection } => {
        task_queries::update_status(update.task_id, update.old_value, connection);
      }
      UpdateTaskTitle { update, connection } => {
        task_queries::update_title(update.task_id, &update.old_value, connection);
      }
    }
  }
}
