use actions::{
  FiltererAction, ForwardAction, ReversableAction, ScrollAction, TaskAction, UndoBufferAction,
};
use commands::ActiveTasksViewCommand;
use std::rc::Rc;
use views::{ActiveTasksView, ActiveTasksViewState};

#[derive(Clone)]
pub enum ActiveTasksViewAction {
  Filterer {
    fa: FiltererAction,
  },
  Scroll {
    sa: ScrollAction,
  },
  Task {
    ta: TaskAction,
  },
  UndoBuffer {
    uba: UndoBufferAction<ActiveTasksViewAction, ActiveTasksViewState>,
  },
}

impl ActiveTasksViewAction {
  pub fn prepare_from_command(
    cmd: ActiveTasksViewCommand,
    view: &ActiveTasksView,
  ) -> Option<ActiveTasksViewAction> {
    use self::ActiveTasksViewCommand::*;

    match cmd {
      Filterer(fc) => fc
        .to_action(&view.root_window, &view.filterer)
        .map(|fa| ActiveTasksViewAction::Filterer { fa }),
      Scroll(sc) => sc
        .to_action(&view.root_window, &view.scroller)
        .map(|sa| ActiveTasksViewAction::Scroll { sa }),
      Task(tc) => {
        let scroller = Rc::clone(&view.scroller);
        tc.to_action(&view.root_window, &view.connection, || {
          scroller.current_task()
        }).map(|ta| ActiveTasksViewAction::Task { ta })
      }
      UndoBuffer(ubc) => {
        let uba = ubc.to_action(&view.undo_buffer);
        Some(ActiveTasksViewAction::UndoBuffer { uba })
      }
    }
  }
}

impl ForwardAction for ActiveTasksViewAction {
  fn execute(&mut self) {
    use self::ActiveTasksViewAction::*;
    match self {
      Filterer { fa } => {
        fa.execute();
      }
      Scroll { sa } => {
        sa.execute();
      }
      Task { ta } => {
        ta.execute();
      }
      UndoBuffer { uba } => {
        uba.execute();
      }
    }
  }
}

impl ReversableAction for ActiveTasksViewAction {
  fn unexecute(&mut self) {
    use self::ActiveTasksViewAction::*;
    match self {
      Filterer { fa } => {
        fa.unexecute();
      }
      Scroll { .. } => panic!("Should not try to unexecute an UnderBufferAction"),
      Task { ta } => {
        ta.unexecute();
      }
      UndoBuffer { .. } => panic!("Should not try to unexecute an UnderBufferAction"),
    }
  }
}
