use models::{TaskDuration, TaskPriority};
use super::data_source::TaskResult;

pub fn score(result: &TaskResult) -> i64 {
  let mut score = result.task_age.num_milliseconds();

  score *= match result.task.priority {
    TaskPriority::Low => 1,
    TaskPriority::Medium => 2,
    TaskPriority::High => 4,
  };

  score *= match result.task.duration {
    TaskDuration::Short => 4,
    TaskDuration::Medium => 2,
    TaskDuration::Long => 1,
  };

  score
}
