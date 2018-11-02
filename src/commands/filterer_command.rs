use actions::{Action, FiltererAction};
use components::Reviewer;

#[derive(Clone, Copy, Debug)]
pub enum FiltererCommand {
  FilterByRequiresInternet,
}

impl FiltererCommand {
  pub fn to_action(self, reviewer: &Reviewer) -> Option<Box<dyn Action>> {
    FiltererAction::prepare_from_cmd(self, reviewer).map(|fa| {
      // Would be nicer if type ascription were not experimental.
      let fa: Box<dyn Action> = Box::new(fa);
      fa
    })
  }
}
