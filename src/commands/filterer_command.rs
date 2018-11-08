use actions::FiltererAction;
use components::Filterer;
use std::rc::Rc;
use util::ui::UserInterface;

#[derive(Clone, Copy, Debug)]
pub enum FiltererCommand {
  FilterByRequiresInternet,
}

impl FiltererCommand {
  pub fn to_action(self, ui: &UserInterface, filterer: &Rc<Filterer>) -> Option<FiltererAction> {
    FiltererAction::prepare_from_cmd(self, ui, filterer)
  }
}
