use components::TasksScroller;

// TODO: I don't like SavedTasksScrolerState. Let's try to get rid of
// it!
#[derive(Clone)]
pub struct SavedTasksScrolerState {
  pub old_id: Option<i32>,
  pub new_id: Option<Option<i32>>,
}

impl SavedTasksScrolerState {
  pub fn new(scroller: &TasksScroller) -> SavedTasksScrolerState {
    SavedTasksScrolerState {
      old_id: scroller.current_task_id(),
      new_id: None,
    }
  }
}
