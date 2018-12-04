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
  fn task_event_is_age_basis_event(task_event: &TaskEvent) -> bool {
    match task_event.event_type {
      TaskEventType::AgeResetRequested => true,
      TaskEventType::DelayRequested => false,
      TaskEventType::TaskEffortRecorded => true,
    }
  }

  fn delay_amount(task_events: &[TaskEvent]) -> f64 {
    assert_is_sorted_backward(task_events);

    // Find and count all delay events since the age basis was set.
    let num_delay_events = task_events
      .iter()
      .take_while(|te| !Scorer::task_event_is_age_basis_event(te))
      .filter(|te| te.event_type == TaskEventType::DelayRequested)
      .count();

    Duration::days(num_delay_events as i64).num_seconds() as f64
  }

  fn last_effort_age_basis(
    task: &Task,
    task_events: &[TaskEvent],
  ) -> DateTime<Utc> {
    assert_is_sorted_backward(task_events);

    // Find most recent event that "reset" the task age basis.
    let latest_task_effort = task_events
      .iter()
      .find(|te| Scorer::task_event_is_age_basis_event(te));

    match latest_task_effort {
      None => task.created_at,
      Some(te) => te.created_at,
    }
  }

  pub fn task_effort_age(task: &Task, task_events: &[TaskEvent], current_time: DateTime<Utc>) -> Duration {
    current_time.signed_duration_since(Scorer::last_effort_age_basis(task, task_events))
  }

  pub fn score_task(
    task: &Task,
    task_events: &[TaskEvent],
    task_effort_age: Duration,
  ) -> i64 {
    let mut score = task_effort_age.num_seconds() as f64;
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
