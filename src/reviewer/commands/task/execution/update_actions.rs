use diesel::pg::PgConnection;
use models::task::queries;
use reviewer::commands::{
  ActionResult,
  task::update_actions::TaskUpdateAction
};

impl TaskUpdateAction {
  pub fn execute(&mut self, connection: &PgConnection) -> ActionResult {
    use self::TaskUpdateAction::*;

    match self {
      UpdateDuration(tvu) => {
        queries::update_duration(tvu.task_id, tvu.new_value, connection);
      }
      UpdatePriority(tvu) => {
        queries::update_priority(tvu.task_id, tvu.new_value, connection);
      }
      UpdateRequiresInternet(tvu) => {
        queries::update_requires_internet(tvu.task_id, tvu.new_value, connection);
      }
      UpdateStatus(tvu) => {
        queries::update_status(tvu.task_id, tvu.new_value, connection);
      }
      UpdateTaskTitle(tvu) => {
        queries::update_title(tvu.task_id, &tvu.new_value, connection);
      }
    }

    ActionResult::DidUpdateTaskData
  }

  pub fn unexecute(&mut self, connection: &PgConnection) -> ActionResult {
    use self::TaskUpdateAction::*;

    match self {
      UpdateDuration(tvu) => {
        queries::update_duration(tvu.task_id, tvu.old_value, connection);
      }
      UpdatePriority(tvu) => {
        queries::update_priority(tvu.task_id, tvu.old_value, connection);
      }
      UpdateRequiresInternet(tvu) => {
        queries::update_requires_internet(tvu.task_id, tvu.old_value, connection);
      }
      UpdateStatus(tvu) => {
        queries::update_status(tvu.task_id, tvu.old_value, connection);
      }
      UpdateTaskTitle(tvu) => {
        queries::update_title(tvu.task_id, &tvu.old_value, connection);
      }
    }

    ActionResult::DidUpdateTaskData
  }
}
