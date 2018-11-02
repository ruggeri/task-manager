use super::{Action, ActionRequest::RequestDataSourceUpdate, TaskAction};
use components::Reviewer;
use queries::{task as task_queries, task_effort as te_queries};

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
        task_effort,
      } => {
        if let Some(task_effort) = task_effort {
          te_queries::update_destroyed(task_effort.id, false, &reviewer.connection);
        } else {
          *task_effort = Some(te_queries::record(*task_id, &reviewer.connection));
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
      RecordTaskEffort { task_effort, .. } => {
        let task_effort = match task_effort {
          None => panic!("Cannot undo a never performed record effort action"),
          Some(task_effort) => task_effort,
        };

        te_queries::update_destroyed(task_effort.id, true, &reviewer.connection);
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
