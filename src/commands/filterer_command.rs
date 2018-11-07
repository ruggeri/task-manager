use actions::FiltererAction;
use components::Filterer;
use std::rc::Rc;
use util::ui::Window;

#[derive(Clone, Copy, Debug)]
pub enum FiltererCommand {
  FilterByRequiresInternet,
}

impl FiltererCommand {
  pub fn to_action(self, window: &Window, filterer: &Rc<Filterer>) -> Option<FiltererAction> {
    use self::FiltererCommand::*;
    match self {
      FilterByRequiresInternet => {
        FiltererAction::prepare_from_cmd(self, window, filterer)
      }
    }
  }
}
