use actions::{ForwardAction, ReversableAction, TaskAction};
use queries::{task as task_queries, task_event as te_queries};

impl ForwardAction for TaskAction {
  fn execute(&mut self) {
    use self::TaskAction::*;

    match self {
      // Create a task.
      CreateTask {
        task_title,
        task,
        connection,
      } => {
        if let Some(task) = task {
          task_queries::update_destroyed(task.id, false, &connection);
        } else {
          *task = Some(task_queries::create(task_title, &connection));
        }
      }

      // Record a task effort.
      RecordTaskEffort {
        task_id,
        task_event,
        connection,
      } => {
        if let Some(task_event) = task_event {
          te_queries::update_destroyed(
            task_event.id,
            false,
            &connection,
          );
        } else {
          *task_event =
            Some(te_queries::record_task_effort(*task_id, &connection));
        }
      }

      // Request a task age reset.
      RequestTaskAgeReset {
        task_id,
        task_event,
        connection,
      } => {
        if let Some(task_event) = task_event {
          te_queries::update_destroyed(
            task_event.id,
            false,
            &connection,
          );
        } else {
          *task_event =
            Some(te_queries::request_task_age_reset(*task_id, &connection));
        }
      }

      // Request a task delay.
      RequestTaskDelay {
        task_id,
        task_event,
        connection,
      } => {
        if let Some(task_event) = task_event {
          te_queries::update_destroyed(
            task_event.id,
            false,
            &connection,
          );
        } else {
          *task_event =
            Some(te_queries::request_delay(*task_id, &connection));
        }
      }

      // Update a task attribute.
      TaskUpdate(update_action) => update_action.execute(),
    }
  }
}

impl ReversableAction for TaskAction {
  fn unexecute(&mut self) {
    use self::TaskAction::*;

    match self {
      // Undo task creation.
      CreateTask {
        task, connection, ..
      } => {
        let task = match task {
          None => panic!("Cannot undo a never performed create action"),
          Some(task) => task,
        };

        task_queries::update_destroyed(task.id, true, &connection);
      }

      // Undo task effort creation.
      RecordTaskEffort {
        task_event,
        connection,
        ..
      } => {
        let task_event = match task_event {
          None => {
            panic!("Cannot undo a never performed record effort action")
          }
          Some(task_event) => task_event,
        };

        te_queries::update_destroyed(task_event.id, true, &connection);
      }

      // Undo age reset request.
      RequestTaskAgeReset {
        task_event,
        connection,
        ..
      } => {
        let task_event = match task_event {
          None => {
            panic!("Cannot undo a never performed request age reset action")
          }
          Some(task_event) => task_event,
        };

        te_queries::update_destroyed(task_event.id, true, &connection);
      }

      // Undo delay request.
      RequestTaskDelay {
        task_event,
        connection,
        ..
      } => {
        let task_event = match task_event {
          None => {
            panic!("Cannot undo a never performed request delay action")
          }
          Some(task_event) => task_event,
        };

        te_queries::update_destroyed(task_event.id, true, &connection);
      }

      // Undo task attribute update.
      TaskUpdate(update_action) => update_action.unexecute(),
    }
  }
}
