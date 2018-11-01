use super::ActionResult;
use components::Reviewer;

pub trait Action {
  fn execute(&mut self, reviewer: &mut Reviewer) -> ActionResult;
  fn unexecute(&mut self, reviewer: &mut Reviewer) -> ActionResult;
  fn can_be_unexecuted(&self) -> bool;
}