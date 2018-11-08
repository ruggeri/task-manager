use chrono::{DateTime, Duration, Utc};
use models::{
  Task, TaskDuration, TaskEvent, TaskEventType, TaskPriority,
};

pub struct Scorer();

fn assert_is_sorted_backward(task_events: &[TaskEvent]) {
  for idx in 1..task_events.len() {
    if task_events[idx - 1].created_at < task_events[idx].created_at {
      panic!("Expected tasks to be sorted.")
    }
  }
}

impl Scorer {
  fn delay_amount(task_events: &[TaskEvent]) -> i64 {
    assert_is_sorted_backward(task_events);

    let num_delay_events = task_events
      .iter()
      .take_while(|te| {
        te.event_type != TaskEventType::TaskEffortRecorded
      }).filter(|te| te.event_type == TaskEventType::DelayRequested)
      .count();

    Duration::days(num_delay_events as i64).num_seconds()
  }

  pub fn last_effort_time(
    task: &Task,
    task_events: &[TaskEvent],
  ) -> DateTime<Utc> {
    assert_is_sorted_backward(task_events);

    // Filter to just TaskEffortRecorded events.
    let latest_task_effort = task_events
      .iter()
      .find(|te| te.event_type == TaskEventType::TaskEffortRecorded);

    match latest_task_effort {
      None => task.created_at,
      Some(te) => te.created_at,
    }
  }

  pub fn score_task(
    task: &Task,
    task_events: &[TaskEvent],
    last_effort_duration_since: Duration,
  ) -> i64 {
    let mut score = last_effort_duration_since.num_seconds();
    score -= Scorer::delay_amount(task_events);

    score *= match task.priority {
      TaskPriority::Low => 1,
      TaskPriority::Medium => 2,
      TaskPriority::High => 4,
    };

    score *= match task.duration {
      TaskDuration::Short => 4,
      TaskDuration::Medium => 2,
      TaskDuration::Long => 1,
    };

    score
  }
}
