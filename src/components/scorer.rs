use chrono::{DateTime, Duration, Utc};
use models::{
  Task, TaskDuration, TaskEvent, TaskEventType, TaskPriority,
};

const BASE_PRIORITY_FACTOR: f64 = 1.42;

pub struct Scorer();

fn assert_is_sorted_backward(task_events: &[TaskEvent]) {
  for idx in 1..task_events.len() {
    if task_events[idx - 1].created_at < task_events[idx].created_at {
      panic!("Expected tasks to be sorted.")
    }
  }
}

impl Scorer {
  fn delay_amount(task_events: &[TaskEvent]) -> f64 {
    assert_is_sorted_backward(task_events);

    let num_delay_events = task_events
      .iter()
      .take_while(|te| {
        te.event_type != TaskEventType::TaskEffortRecorded
      }).filter(|te| te.event_type == TaskEventType::DelayRequested)
      .count();

    Duration::days(num_delay_events as i64).num_seconds() as f64
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
    let mut score = last_effort_duration_since.num_seconds() as f64;
    score -= Scorer::delay_amount(task_events);

    score *= match task.priority {
      TaskPriority::Low => 1.0,
      TaskPriority::Medium => BASE_PRIORITY_FACTOR,
      TaskPriority::High => BASE_PRIORITY_FACTOR * BASE_PRIORITY_FACTOR,
    };

    score *= match task.duration {
      TaskDuration::Short => {
        BASE_PRIORITY_FACTOR * BASE_PRIORITY_FACTOR
      }
      TaskDuration::Medium => BASE_PRIORITY_FACTOR,
      TaskDuration::Long => 1.0,
    };

    score as i64
  }
}
