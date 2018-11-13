use components::TasksScroller;

#[derive(Clone, Copy)]
pub enum NewScrollerTaskId {
  NeverSaved,
  Saved(Option<i32>),
}

#[derive(Clone, Copy)]
pub struct SavedTasksScrolerState {
  pub old_id: Option<i32>,
  pub new_id: NewScrollerTaskId
}

impl SavedTasksScrolerState {
  pub fn new(scroller: &TasksScroller) -> SavedTasksScrolerState {
    SavedTasksScrolerState {
      old_id: scroller.current_task_id(),
      new_id: NewScrollerTaskId::NeverSaved,
    }
  }

  pub fn unwrap_new_id(self: SavedTasksScrolerState) -> Option<i32> {
    use self::NewScrollerTaskId::*;
    match self.new_id {
      NeverSaved => panic!("Scroller state was never saved!"),
      Saved(new_id) => new_id
    }
  }
}
