use super::{Action, ActionRequest::RequestDataSourceUpdate, TaskAction};
use components::Reviewer;
use queries::{task as task_queries, task_event as te_queries};

impl Action for TaskAction {
  fn execute(&mut self, reviewer: &Reviewer) {
    use self::TaskAction::*;

    match self {
      // Create a task.
      CreateTask { task_title, task } => {
        if let Some(task) = task {
          task_queries::update_destroyed(task.id, false, &reviewer.connection);
        } else {
          *task = Some(task_queries::create(task_title, &reviewer.connection));
        }

        reviewer.execute_action_request(RequestDataSourceUpdate);
      }

      // Record a task effort.
      RecordTaskEffort {
        task_id,
        task_event,
      } => {
        if let Some(task_event) = task_event {
          te_queries::update_destroyed(task_event.id, false, &reviewer.connection);
        } else {
          *task_event = Some(te_queries::record_task_effort(*task_id, &reviewer.connection));
        }

        reviewer.execute_action_request(RequestDataSourceUpdate);
      }

      // Update a task attribute.
      TaskUpdate(update_action) => update_action.execute(&reviewer),
    }
  }

  fn unexecute(&mut self, reviewer: &Reviewer) {
    use self::TaskAction::*;

    match self {
      // Undo task creation.
      CreateTask { task, .. } => {
        let task = match task {
          None => panic!("Cannot undo a never performed create action"),
          Some(task) => task,
        };

        task_queries::update_destroyed(task.id, true, &reviewer.connection);
        reviewer.execute_action_request(RequestDataSourceUpdate);
      }

      // Undo task effort creation.
      RecordTaskEffort { task_event, .. } => {
        let task_event = match task_event {
          None => panic!("Cannot undo a never performed record effort action"),
          Some(task_event) => task_event,
        };

        te_queries::update_destroyed(task_event.id, true, &reviewer.connection);
        reviewer.execute_action_request(RequestDataSourceUpdate);
      }

      // Undo task attribute update.
      TaskUpdate(update_action) => update_action.unexecute(&reviewer),
    }
  }

  fn can_be_unexecuted(&self) -> bool {
    true
  }
}
