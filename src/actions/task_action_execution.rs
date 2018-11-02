use super::{Action, ActionResult, TaskAction};
use components::Reviewer;
use queries::{task as task_queries, task_effort as te_queries};

impl Action for TaskAction {
  fn execute(&mut self, reviewer: &Reviewer) -> ActionResult {
    use self::TaskAction::*;

    let connection = &reviewer.connection;

    match self {
      // Create a task.
      CreateTask { task_title, task } => {
        if let Some(task) = task {
          task_queries::update_destroyed(task.id, false, connection);
        } else {
          *task = Some(task_queries::create(task_title, connection));
        }

        ActionResult::DidUpdateTaskData
      }

      // Record a task effort.
      RecordTaskEffort {
        task_id,
        task_effort,
      } => {
        if let Some(task_effort) = task_effort {
          te_queries::update_destroyed(task_effort.id, false, connection);
        } else {
          *task_effort = Some(te_queries::record(*task_id, connection));
        }

        ActionResult::DidUpdateTaskData
      }

      // Update a task attribute.
      TaskUpdate(update_action) => update_action.execute(connection),
    }
  }

  fn unexecute(&mut self, reviewer: &Reviewer) -> ActionResult {
    use self::TaskAction::*;

    let connection = &reviewer.connection;

    match self {
      // Undo task creation.
      CreateTask { task, .. } => {
        let task = match task {
          None => panic!("Cannot undo a never performed create action"),
          Some(task) => task,
        };

        task_queries::update_destroyed(task.id, true, connection);
        ActionResult::DidUpdateTaskData
      }

      // Undo task effort creation.
      RecordTaskEffort { task_effort, .. } => {
        let task_effort = match task_effort {
          None => panic!("Cannot undo a never performed record effort action"),
          Some(task_effort) => task_effort,
        };

        te_queries::update_destroyed(task_effort.id, true, connection);
        ActionResult::DidUpdateTaskData
      }

      // Undo task attribute update.
      TaskUpdate(update_action) => update_action.unexecute(connection),
    }
  }

  fn can_be_unexecuted(&self) -> bool {
    true
  }
}
