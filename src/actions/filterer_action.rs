use actions::{Action, ActionResult};
use commands::FiltererCommand;
use components::Reviewer;

#[derive(Clone, Debug)]
pub struct FilterValueUpdate<T: Eq> {
  pub old_value: T,
  pub new_value: T,
}

impl<T: Eq> FilterValueUpdate<T> {
  pub fn new(old_value: T, new_value: T) -> Option<FilterValueUpdate<T>> {
    if old_value == new_value {
      None
    } else {
      Some(FilterValueUpdate { old_value, new_value })
    }
  }
}

pub enum FiltererAction {
  UpdateFilterByRequiresInternet(FilterValueUpdate<Option<bool>>),
}

#[allow(option_option)]
fn get_requires_internet_value(reviewer: &Reviewer) -> Option<Option<bool>> {
  let str_value = reviewer.window.read_line("Requires internet value: ");
  match &str_value as &str {
    "yes" => Some(Some(true)),
    "no" => Some(Some(false)),
    "any" => Some(None),
    _ => None
  }
}

impl FiltererAction {
  pub fn prepare_from_cmd(
    cmd: FiltererCommand,
    reviewer: &Reviewer,
  ) -> Option<FiltererAction> {
    use self::FiltererCommand as FCmd;
    use self::FiltererAction as FAction;

    match cmd {
      FCmd::FilterByRequiresInternet => {
        get_requires_internet_value(reviewer).and_then(|new_value| {
          let old_value = reviewer.filterer.requires_internet_value.get();
          FilterValueUpdate::new(old_value, new_value).map(|fvu| {
            FAction::UpdateFilterByRequiresInternet(fvu)
          })
        })
      }
    }
  }
}

impl Action for FiltererAction {
  fn execute(&mut self, reviewer: &Reviewer) -> ActionResult {
    use self::FiltererAction::*;

    match self {
      UpdateFilterByRequiresInternet(update) => {
        reviewer.filterer.requires_internet_value.set(update.new_value)
      }
    }

    ActionResult::DidUpdateFilterer
  }

  fn unexecute(&mut self, reviewer: &Reviewer) -> ActionResult {
    use self::FiltererAction::*;

    match self {
      UpdateFilterByRequiresInternet(update) => {
        reviewer.filterer.requires_internet_value.set(update.old_value)
      }
    }

    ActionResult::DidUpdateFilterer
  }

  fn can_be_unexecuted(&self) -> bool {
    true
  }
}
