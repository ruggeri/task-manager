use commands::TaskUpdateCommand;
use diesel::pg::PgConnection;
use models::*;
use std::rc::Rc;
use util::ui::Window;

macro_rules! define_task_update_action {
  ( $( ($value:ident, $type:ty) ),* ) => {
    #[derive(Clone)]
    pub enum TaskUpdateAction { $(
      $value {
        task_id: i32,
        old_value: $type,
        new_value: $type,
        connection: Rc<PgConnection>,
      },
    )* }
  }
}

define_task_update_action!(
  (UpdateDuration, TaskDuration),
  (UpdatePriority, TaskPriority),
  (UpdateRequiresInternet, bool),
  (UpdateStatus, TaskStatus),
  (UpdateTaskTitle, String)
);

macro_rules! prepare_action {
  ( $enum_value:ident, $task_id:expr, $old_value:expr, $new_value:expr, $connection:expr ) => {
    if $old_value == $new_value {
      None
    } else {
      Some(TaskUpdateAction::$enum_value {
        task_id: $task_id,
        old_value: $old_value,
        new_value: $new_value,
        connection: Rc::clone($connection),
      })
    }
  };
}

impl TaskUpdateAction {
  pub fn prepare_from_cmd(
    cmd: TaskUpdateCommand,
    task: &Task,
    window: &Window,
    connection: &Rc<PgConnection>,
  ) -> Option<TaskUpdateAction> {
    use self::TaskUpdateCommand as Cmd;

    match cmd {
      // Edit a task title.
      Cmd::EditTaskTitle => {
        let new_task_title = match window.read_line("Edit task title: ") {
          // If they hit Ctrl-C don't make the task afterall.
          None => return None,
          Some(new_task_title) => new_task_title,
        };

        prepare_action!(
          UpdateTaskTitle,
          task.id,
          task.title.clone(),
          new_task_title,
          connection
        )
      }

      // Toggle whether a task requires internet.
      Cmd::ToggleRequiresInternet => prepare_action!(
        UpdateRequiresInternet,
        task.id,
        task.requires_internet,
        !task.requires_internet,
        connection
      ),

      // Update a task's duration.
      Cmd::UpdateDuration(direction) => prepare_action!(
        UpdateDuration,
        task.id,
        task.duration,
        task.duration.increment(direction),
        connection
      ),

      // Update a task's priority.
      Cmd::UpdatePriority(direction) => prepare_action!(
        UpdatePriority,
        task.id,
        task.priority,
        task.priority.increment(direction),
        connection
      ),

      // Update a task's status.
      Cmd::UpdateStatus(new_task_status) => prepare_action!(
        UpdateStatus,
        task.id,
        task.status,
        new_task_status,
        connection
      ),
    }
  }
}
