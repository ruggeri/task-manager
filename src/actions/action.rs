use super::ActionResult;
use components::Reviewer;

pub trait Action {
  fn execute(&mut self, reviewer: &Reviewer) -> ActionResult;
  fn unexecute(&mut self, reviewer: &Reviewer) -> ActionResult;
  fn can_be_unexecuted(&self) -> bool;
}
