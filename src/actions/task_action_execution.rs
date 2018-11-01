use components::Reviewer;
use models::{
  task::queries,
  task_effort::TaskEffort,
};
use super::{Action, ActionResult, TaskAction};

impl Action for TaskAction {
  fn execute(&mut self, reviewer: &mut Reviewer) -> ActionResult {
    use self::TaskAction::*;

    let connection = &reviewer.connection;

    match self {
      CreateTask{ task_title, task } => {
        if task.is_some() {
          panic!("Cannot redo a create action twice");
        }

        *task = Some(queries::create(task_title, connection));
        ActionResult::DidUpdateTaskData
      }
      RecordTaskEffort{ task_id, task_effort } => {
        if task_effort.is_some() {
          panic!("Cannot redo a record effort action twice");
        }

        *task_effort = Some(TaskEffort::record(*task_id, connection));
        ActionResult::DidUpdateTaskData
      }
      TaskUpdate(update_action) => update_action.execute(connection)
    }
  }

  fn unexecute(&mut self, reviewer: &mut Reviewer) -> ActionResult {
    use self::TaskAction::*;

    let connection = &reviewer.connection;

    match self {
      CreateTask{ task, .. } => {
        if task.is_none() {
          panic!("Cannot undo a never performed create action");
        }

        let task_id = task.take().unwrap().id;
        queries::destroy(task_id, connection);
        ActionResult::DidUpdateTaskData
      }
      RecordTaskEffort{ task_effort, .. } => {
        if task_effort.is_none() {
          panic!("Cannot undo a never performed record effort action");
        }

        let task_effort_id = task_effort.take().unwrap().id;
        TaskEffort::destroy(task_effort_id, connection);
        ActionResult::DidUpdateTaskData
      }
      TaskUpdate(update_action) => update_action.unexecute(connection)
    }
  }

  fn can_be_unexecuted(&self) -> bool {
    true
  }
}
