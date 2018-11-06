use actions::{Action, FiltererAction};
use application::Application;

#[derive(Clone, Copy, Debug)]
pub enum FiltererCommand {
  FilterByRequiresInternet,
}

impl FiltererCommand {
  pub fn to_action(self, application: &Application) -> Option<Box<dyn Action>> {
    FiltererAction::prepare_from_cmd(self, application).map(|fa| {
      // Would be nicer if type ascription were not experimental.
      let fa: Box<dyn Action> = Box::new(fa);
      fa
    })
  }
}
