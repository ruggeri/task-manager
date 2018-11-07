use actions::{FiltererAction, ScrollAction, TaskAction, UndoBufferAction};
use commands::ActiveTasksViewCommand;
use views::ActiveTasksView;
use std::rc::Rc;

#[derive(Clone)]
pub enum ActiveTasksViewAction {
  Filterer(FiltererAction),
  Scroll(ScrollAction),
  Task(TaskAction),
  UndoBuffer(UndoBufferAction),
}

impl ActiveTasksViewAction {
  pub fn prepare_from_command(cmd: ActiveTasksViewCommand, view: &mut ActiveTasksView) -> Option<ActiveTasksViewAction> {
    use self::ActiveTasksViewCommand::*;

    match cmd {
      Filterer(fc) => {
        fc.to_action(
          &view.root_window,
          &view.filterer
        ).map(|fa| ActiveTasksViewAction::Filterer(fa))
      }
      Scroll(sc) => {
        sc.to_action(
          &view.root_window,
          &view.scroller,
        ).map(|sa| ActiveTasksViewAction::Scroll(sa))
      }
      Task(tc) => {
        let scroller = Rc::clone(&view.scroller);
        tc.to_action(
          &view.root_window,
          &view.connection,
          || scroller.current_task()
        ).map(|ta| ActiveTasksViewAction::Task(ta))
      }
      UndoBuffer(ubc) => {
        let uba = ubc.to_action(&view.undo_buffer);
        Some(ActiveTasksViewAction::UndoBuffer(uba))
      },
    }
  }
}
