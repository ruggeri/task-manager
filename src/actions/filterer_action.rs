use actions::{ForwardAction, ReversableAction};
use commands::FiltererCommand;
use components::{
  AttributeFilter,
  filterer::RequiresInternetFiltererValue
};
use std::rc::Rc;
use util::ui::Window;

#[derive(Clone)]
pub enum FiltererAction {
  UpdateRequiresInternet {
    new_value: RequiresInternetFiltererValue,
    filterer: Rc<AttributeFilter>,
  },
}

impl ForwardAction for FiltererAction {
  fn execute(&mut self) {
    use self::FiltererAction::*;

    match self {
      UpdateRequiresInternet { new_value, filterer, .. } => {
        filterer.requires_internet_value.set(*new_value);
      }
    }
  }
}

impl ReversableAction for FiltererAction {
  fn unexecute(&mut self) {
    use self::FiltererAction::*;

    match self {
      UpdateRequiresInternet { .. } => {
        // Nothing special to undo. Simply restore the prior state.
      }
    }
  }
}

#[allow(option_option)]
fn read_requires_internet_value(window: &Window) -> Option<RequiresInternetFiltererValue> {
  let str_value = window.read_line("Requires internet value: ");

  use self::RequiresInternetFiltererValue::*;
  // Gross. Cannot compare a String with a &str. So need to call
  // `s.as_ref()`. But then also Option<String> will give `map` a
  // String, and we want it to be giving the `&String` reference. Ugh.
  match str_value.as_ref().map(|s| s.as_ref()) {
    Some("yes") => Some(Yes),
    Some("no") => Some(No),
    Some("any") => Some(Any),
    // Includes Ctrl-C
    _ => None
  }
}

fn new_requires_internet_filterer_action(window: &Window, filterer: &Rc<AttributeFilter>) -> Option<FiltererAction> {
  let old_value = filterer.requires_internet_value.get();
  let new_value = match read_requires_internet_value(window) {
    None => return None,
    Some(new_value) => new_value
  };

  if old_value == new_value {
    return None;
  }

  let fa = FiltererAction::UpdateRequiresInternet {
    new_value,
    filterer,
  };

  Some(fa)
}

impl FiltererAction {
  pub fn prepare_from_cmd(
    cmd: FiltererCommand,
    window: &Window,
    filterer: &Rc<AttributeFilter>,
  ) -> Option<FiltererAction> {
    use self::FiltererCommand::*;

    match cmd {
      FilterByRequiresInternet => {
        new_requires_internet_filterer_action(window, filterer)
      }
    }
  }
}
