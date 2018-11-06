use actions::{
  Action,
  ActionRequest::{RequestFiltererUpdate, RequestScrollerUpdate}
};
use commands::FiltererCommand;
use application::Application;

#[derive(Clone, Debug)]
pub struct FilterValueUpdate<T: Eq> {
  pub old_value: T,
  pub old_result_idx: i32,
  pub new_value: T,
  pub new_result_idx: i32,
}

impl<T: Eq> FilterValueUpdate<T> {
  pub fn new(old_value: T, new_value: T) -> Option<FilterValueUpdate<T>> {
    if old_value == new_value {
      None
    } else {
      Some(FilterValueUpdate {
        old_value,
        old_result_idx: 0,
        new_value,
        new_result_idx: 0
      })
    }
  }
}

pub enum FiltererAction {
  UpdateFilterByRequiresInternet(FilterValueUpdate<Option<bool>>),
}

#[allow(option_option)]
fn get_requires_internet_value(application: &Application) -> Option<Option<bool>> {
  let str_value = application.window.read_line("Requires internet value: ");

  // Gross. Cannot compare a String with a &str. So need to call
  // `s.as_ref()`. But then also Option<String> will give `map` a
  // String, and we want it to be giving the `&String` reference. Ugh.
  match str_value.as_ref().map(|s| s.as_ref()) {
    Some("yes") => Some(Some(true)),
    Some("no") => Some(Some(false)),
    Some("any") => Some(None),
    // Includes Ctrl-C
    _ => None
  }
}

impl FiltererAction {
  pub fn prepare_from_cmd(
    cmd: FiltererCommand,
    application: &Application,
  ) -> Option<FiltererAction> {
    use self::FiltererCommand as FCmd;
    use self::FiltererAction as FAction;

    match cmd {
      FCmd::FilterByRequiresInternet => {
        get_requires_internet_value(application).and_then(|new_value| {
          let old_value = application.filterer.requires_internet_value.get();
          FilterValueUpdate::new(old_value, new_value).map(|fvu| {
            FAction::UpdateFilterByRequiresInternet(fvu)
          })
        })
      }
    }
  }
}

impl Action for FiltererAction {
  fn execute(&mut self, application: &Application) {
    use self::FiltererAction::*;

    match self {
      UpdateFilterByRequiresInternet(update) => {
        update.old_result_idx = application.scroller.current_result_idx();
        application.filterer.requires_internet_value.set(update.new_value);
        application.execute_action_request(RequestFiltererUpdate);
        application.scroller.set_current_result_idx(update.new_result_idx);
        application.execute_action_request(RequestScrollerUpdate);
      }
    }
  }

  fn unexecute(&mut self, application: &Application) {
    use self::FiltererAction::*;

    match self {
      UpdateFilterByRequiresInternet(update) => {
        update.new_result_idx = application.scroller.current_result_idx();
        application.filterer.requires_internet_value.set(update.old_value);
        application.execute_action_request(RequestFiltererUpdate);
        application.scroller.set_current_result_idx(update.old_result_idx);
        application.execute_action_request(RequestScrollerUpdate);
      }
    }
  }

  fn can_be_unexecuted(&self) -> bool {
    true
  }
}
