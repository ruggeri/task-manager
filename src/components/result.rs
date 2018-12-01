use components::Scorer;
use diesel::pg::PgConnection;
use models::{Task, TaskEvent};
use queries::task_event as te_queries;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Clone, Debug)]
pub struct TaskResult {
  pub task: Task,
  pub task_events: Vec<TaskEvent>,
  pub task_effort_age: ::chrono::Duration,
  pub score: i64,
}

impl TaskResult {
  pub fn from_task(
    task: Task,
    current_time: DateTime,
    connection: &PgConnection,
  ) -> TaskResult {
    let task_events = te_queries::task_events(&task, connection);
    let task_effort_age = Scorer::task_effort_age(&task, &task_events, current_time);
    let score = Scorer::score_task(
      &task,
      &task_events,
      task_effort_age,
    );

    TaskResult {
      task,
      task_events,
      task_effort_age,
      score,
    }
  }
}

#[derive(Clone, Debug)]
pub struct TaskEventResult {
  pub task: Task,
  pub task_event: TaskEvent,
}
