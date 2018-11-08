use actions::TaskUpdateAction;
use queries::task as task_queries;

// TODO: Insane level of duplication. Macro time?
impl TaskUpdateAction {
  pub fn execute(&mut self) {
    use self::TaskUpdateAction::*;
    match self {
      UpdateDuration {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_duration(*task_id, update.new_value, connection);
      }
      UpdatePriority {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_priority(*task_id, update.new_value, connection);
      }
      UpdateRequiresInternet {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_requires_internet(*task_id, update.new_value, connection);
      }
      UpdateStatus {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_status(*task_id, update.new_value, connection);
      }
      UpdateTaskTitle {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_title(*task_id, &update.new_value, connection);
      }
    }
  }

  pub fn unexecute(&mut self) {
    use self::TaskUpdateAction::*;
    match self {
      UpdateDuration {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_duration(*task_id, update.old_value, connection);
      }
      UpdatePriority {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_priority(*task_id, update.old_value, connection);
      }
      UpdateRequiresInternet {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_requires_internet(*task_id, update.old_value, connection);
      }
      UpdateStatus {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_status(*task_id, update.old_value, connection);
      }
      UpdateTaskTitle {
        task_id,
        update,
        connection,
      } => {
        task_queries::update_title(*task_id, &update.old_value, connection);
      }
    }
  }
}
