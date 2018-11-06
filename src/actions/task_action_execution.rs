use super::{Action, ActionRequest::RequestDataSourceUpdate, TaskAction};
use application::Application;
use queries::{task as task_queries, task_event as te_queries};

impl Action for TaskAction {
  fn execute(&mut self, application: &Application) {
    use self::TaskAction::*;

    match self {
      // Create a task.
      CreateTask { task_title, task } => {
        if let Some(task) = task {
          task_queries::update_destroyed(task.id, false, &application.connection);
        } else {
          *task = Some(task_queries::create(task_title, &application.connection));
        }

        application.execute_action_request(RequestDataSourceUpdate);
      }

      // Record a task effort.
      RecordTaskEffort {
        task_id,
        task_event,
      } => {
        if let Some(task_event) = task_event {
          te_queries::update_destroyed(task_event.id, false, &application.connection);
        } else {
          *task_event = Some(te_queries::record_task_effort(*task_id, &application.connection));
        }

        application.execute_action_request(RequestDataSourceUpdate);
      }

      // Request a task delay.
      RequestTaskDelay {
        task_id,
        task_event,
      } => {
        if let Some(task_event) = task_event {
          te_queries::update_destroyed(task_event.id, false, &application.connection);
        } else {
          *task_event = Some(te_queries::request_delay(*task_id, &application.connection));
        }

        application.execute_action_request(RequestDataSourceUpdate);
      }

      // Update a task attribute.
      TaskUpdate(update_action) => update_action.execute(&application),
    }
  }

  fn unexecute(&mut self, application: &Application) {
    use self::TaskAction::*;

    match self {
      // Undo task creation.
      CreateTask { task, .. } => {
        let task = match task {
          None => panic!("Cannot undo a never performed create action"),
          Some(task) => task,
        };

        task_queries::update_destroyed(task.id, true, &application.connection);
        application.execute_action_request(RequestDataSourceUpdate);
      }

      // Undo task effort creation.
      RecordTaskEffort { task_event, .. } => {
        let task_event = match task_event {
          None => panic!("Cannot undo a never performed record effort action"),
          Some(task_event) => task_event,
        };

        te_queries::update_destroyed(task_event.id, true, &application.connection);
        application.execute_action_request(RequestDataSourceUpdate);
      }

      // Undo delay request.
      RequestTaskDelay { task_event, .. } => {
        let task_event = match task_event {
          None => panic!("Cannot undo a never performed request delay action"),
          Some(task_event) => task_event,
        };

        te_queries::update_destroyed(task_event.id, true, &application.connection);
        application.execute_action_request(RequestDataSourceUpdate);
      }

      // Undo task attribute update.
      TaskUpdate(update_action) => update_action.unexecute(&application),
    }
  }

  fn can_be_unexecuted(&self) -> bool {
    true
  }
}
