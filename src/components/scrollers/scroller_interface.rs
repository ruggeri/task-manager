use models::{Direction, End};

pub trait Scroller {
  fn current_result_idx(&self) -> i32;
  fn jump(&self, end: End);
  fn num_results(&self) -> i32;
  fn scroll(&self, direction: Direction);
  fn set_current_result_idx(&self, new_result_idx: i32);
}

