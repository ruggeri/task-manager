use actions::{ActionRequest::RequestDataSourceUpdate, TaskUpdateAction};
use application::Application;
use queries::task as task_queries;

// TODO: Insane level of duplication. Macro time?
impl TaskUpdateAction {
  pub fn execute(&mut self, application: &Application) {
    use self::TaskUpdateAction::*;

    let connection = &application.connection;
    match self {
      UpdateDuration(tvu) => {
        task_queries::update_duration(tvu.task_id, tvu.new_value, connection);
      }
      UpdatePriority(tvu) => {
        task_queries::update_priority(tvu.task_id, tvu.new_value, connection);
      }
      UpdateRequiresInternet(tvu) => {
        task_queries::update_requires_internet(tvu.task_id, tvu.new_value, connection);
      }
      UpdateStatus(tvu) => {
        task_queries::update_status(tvu.task_id, tvu.new_value, connection);
      }
      UpdateTaskTitle(tvu) => {
        task_queries::update_title(tvu.task_id, &tvu.new_value, connection);
      }
    }

    application.execute_action_request(RequestDataSourceUpdate);
  }

  pub fn unexecute(&mut self, application: &Application) {
    use self::TaskUpdateAction::*;

    let connection = &application.connection;
    match self {
      UpdateDuration(tvu) => {
        task_queries::update_duration(tvu.task_id, tvu.old_value, connection);
      }
      UpdatePriority(tvu) => {
        task_queries::update_priority(tvu.task_id, tvu.old_value, connection);
      }
      UpdateRequiresInternet(tvu) => {
        task_queries::update_requires_internet(tvu.task_id, tvu.old_value, connection);
      }
      UpdateStatus(tvu) => {
        task_queries::update_status(tvu.task_id, tvu.old_value, connection);
      }
      UpdateTaskTitle(tvu) => {
        task_queries::update_title(tvu.task_id, &tvu.old_value, connection);
      }
    }

    application.execute_action_request(RequestDataSourceUpdate);
  }
}
