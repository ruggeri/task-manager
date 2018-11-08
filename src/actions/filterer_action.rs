use actions::{ForwardAction, ReversableAction};
use commands::FiltererCommand;
use components::{filterer::FiltererRequiresInternetValue, Filterer};
use std::rc::Rc;
use util::ui::UserInterface;

#[derive(Clone)]
pub enum FiltererAction {
  UpdateRequiresInternet {
    new_value: FiltererRequiresInternetValue,
    old_value: FiltererRequiresInternetValue,
    filterer: Rc<Filterer>,
  },
}

impl ForwardAction for FiltererAction {
  fn execute(&mut self) {
    use self::FiltererAction::*;

    match self {
      UpdateRequiresInternet {
        new_value,
        filterer,
        ..
      } => {
        filterer.set_requires_internet_value(*new_value);
      }
    }
  }
}

impl ReversableAction for FiltererAction {
  fn unexecute(&mut self) {
    use self::FiltererAction::*;

    match self {
      UpdateRequiresInternet {
        old_value,
        filterer,
        ..
      } => {
        filterer.set_requires_internet_value(*old_value);
      }
    }
  }
}

fn read_requires_internet_value(ui: &UserInterface) -> Option<FiltererRequiresInternetValue> {
  let str_value = ui.read_line("Requires internet value: ");

  use self::FiltererRequiresInternetValue::*;
  // Gross. Cannot compare a String with a &str. So need to call
  // `s.as_ref()`. But then also Option<String> will give `map` a
  // String, and we want it to be giving the `&String` reference. Ugh.
  match str_value.as_ref().map(|s| s.as_ref()) {
    Some("yes") => Some(Yes),
    Some("no") => Some(No),
    Some("any") => Some(Any),
    // Includes Ctrl-C
    _ => None,
  }
}

fn new_requires_internet_filterer_action(
  ui: &UserInterface,
  filterer: &Rc<Filterer>,
) -> Option<FiltererAction> {
  read_requires_internet_value(ui).and_then(|new_value| {
    let old_value = filterer.requires_internet_value();
    if old_value == new_value {
      None
    } else {
      Some(FiltererAction::UpdateRequiresInternet {
        new_value,
        old_value,
        filterer: Rc::clone(filterer),
      })
    }
  })
}

impl FiltererAction {
  pub fn prepare_from_cmd(
    cmd: FiltererCommand,
    ui: &UserInterface,
    filterer: &Rc<Filterer>,
  ) -> Option<FiltererAction> {
    use self::FiltererCommand::*;
    match cmd {
      FilterByRequiresInternet => new_requires_internet_filterer_action(ui, filterer),
    }
  }
}
