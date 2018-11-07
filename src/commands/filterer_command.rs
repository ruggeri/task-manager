use actions::FiltererAction;
use components::AttributeFilter;
use std::rc::Rc;
use util::ui::Window;

#[derive(Clone, Copy, Debug)]
pub enum FiltererCommand {
  FilterByRequiresInternet,
}

impl FiltererCommand {
  pub fn to_action(self, window: &Window, filterer: &Rc<AttributeFilter>) -> Option<FiltererAction> {
    use self::FiltererCommand::*;
    match self {
      FilterByRequiresInternet => {
        FiltererAction::prepare_from_cmd(self, window, filterer)
      }
    }
  }
}
