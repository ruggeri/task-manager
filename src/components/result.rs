use components::Scorer;
use diesel::pg::PgConnection;
use models::{Task, TaskEvent};
use queries::task_event as te_queries;

type DateTime = ::chrono::DateTime<::chrono::Utc>;

#[derive(Clone, Debug)]
pub struct TaskResult {
  pub task: Task,
  pub task_events: Vec<TaskEvent>,
  pub last_effort_time: DateTime,
  pub last_effort_duration_since: ::chrono::Duration,
  pub score: i64,
}

impl TaskResult {
  pub fn new(
    task: Task,
    task_events: Vec<TaskEvent>,
    last_effort_time: DateTime,
    last_effort_duration_since: ::chrono::Duration,
    score: i64) -> TaskResult {

    TaskResult {
      task,
      task_events,
      last_effort_time,
      last_effort_duration_since,
      score,
    }
  }

  pub fn from_task(task: Task, current_time: DateTime, connection: &PgConnection) -> TaskResult {
    let task_events = te_queries::task_events(&task, connection);
    let last_effort_time =
      Scorer::last_effort_time(&task, &task_events);
    let last_effort_duration_since =
      current_time.signed_duration_since(last_effort_time);
    let score = Scorer::score_task(
      &task,
      &task_events,
      last_effort_duration_since,
    );

    TaskResult::new(
      task,
      task_events,
      last_effort_time,
      last_effort_duration_since,
      score,
    )
  }
}

#[derive(Clone, Debug)]
pub struct TaskEventResult {
  pub task: Task,
  pub task_event: TaskEvent,
}

#[derive(Clone, Debug)]
pub enum DataResult {
  Task(TaskResult),
  TaskEffort(TaskEventResult),
}

impl DataResult {
  pub fn unwrap_task_result(&self) -> &TaskResult {
    match self {
      DataResult::Task(tr) => tr,
      _ => panic!("Expected TaskResult!")
    }
  }
}
