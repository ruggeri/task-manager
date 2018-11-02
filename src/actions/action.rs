use components::Reviewer;

pub trait Action {
  fn execute(&mut self, reviewer: &Reviewer);
  fn unexecute(&mut self, reviewer: &Reviewer);
  fn can_be_unexecuted(&self) -> bool;
}
