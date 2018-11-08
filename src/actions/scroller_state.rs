use components::Scroller;

#[derive(Clone)]
pub struct SavedScrolerState {
  pub old_id: Option<i32>,
  pub new_id: Option<Option<i32>>,
}

impl SavedScrolerState {
  pub fn new(scroller: &Scroller) -> SavedScrolerState {
    SavedScrolerState {
      old_id: scroller.current_task_id(),
      new_id: None,
    }
  }
}
