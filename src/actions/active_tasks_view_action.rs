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
  pub fn prepare_from_cmd(cmd: ActiveTasksViewCommand, view: &mut ActiveTasksView) -> Option<ActiveTasksViewAction> {
    use self::ActiveTasksViewCommand::*;

    match cmd {
      Filterer(fc) => {
        fc.to_action(
          view.task_results_window.window,
          &view.filterer
        ).and_then(|fa| ActiveTasksViewAction::Filter(fa))
      }
      Scroll(sc) => {
        sc.to_action(
          view.task_results_window.window,
          &view.scroller,
        ).and_then(|sa| ActiveTasksViewAction::Scroll(sa))
      }
      Task(tc) => {
        let scroller = Rc::clone(view.scroller);
        tc.to_action(
          view.task_results_window.window,
          &view.connection,
          || scroller.current_task()
        ).and_then(|ta| ActiveTasksViewAction::Task(ta))
      }
      UndoBuffer(ubc) => {
        ubc.to_action(&view.undo_buffer).and_then(|uba| {
          ActiveTasksViewAction::UndoBuffer(uba)
        })
      },
    }
  }

}
