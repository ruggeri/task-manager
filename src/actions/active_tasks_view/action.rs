use super::{
  execution_logic, saved_scroller_state::SavedTasksScrolerState,
};
use actions::{
  FiltererAction, ForwardAction, ReversableAction, ScrollAction,
  TaskAction, TasksScrollAction, UndoBufferAction,
};
use commands::ActiveTasksViewCommand;
use components::{Scroller, UndoBuffer};
use std::rc::{Rc, Weak};
use views::ActiveTasksView;

#[derive(Clone)]
pub enum ActiveTasksViewAction {
  Filterer {
    fa: FiltererAction,
    view: Weak<ActiveTasksView>,
    scroller_state: SavedTasksScrolerState,
  },
  Scroll {
    sa: ScrollAction,
    view: Weak<ActiveTasksView>,
  },
  Task {
    ta: TaskAction,
    view: Weak<ActiveTasksView>,
    scroller_state: SavedTasksScrolerState,
  },
  TasksScroll {
    tsa: TasksScrollAction,
    view: Weak<ActiveTasksView>,
  },
  UndoBuffer {
    uba: UndoBufferAction,
  },
}

impl ActiveTasksViewAction {
  pub fn prepare_from_command(
    cmd: ActiveTasksViewCommand,
    view: &Rc<ActiveTasksView>,
  ) -> Option<ActiveTasksViewAction> {
    use self::ActiveTasksViewCommand::*;

    match cmd {
      Filterer(fc) => {
        fc.to_action(&view.ui, &view.filterer).map(|fa| {
          ActiveTasksViewAction::Filterer {
            fa,
            view: Rc::downgrade(&view),
            scroller_state: SavedTasksScrolerState::new(&view.scroller),
          }
        })
      }

      Scroll(sc) => {
        let scroller = Rc::clone(&view.scroller) as Rc<Scroller>;

        sc.to_action(&scroller).map(|sa| {
          ActiveTasksViewAction::Scroll {
            sa,
            view: Rc::downgrade(&Rc::clone(view)),
          }
        })
      }

      Task(tc) => tc
        .to_action(&view.ui, &view.connection, || {
          view.scroller.current_task()
        }).map(|ta| ActiveTasksViewAction::Task {
          ta,
          view: Rc::downgrade(&Rc::clone(view)),
          scroller_state: SavedTasksScrolerState::new(&view.scroller),
        }),

      TasksScroll(tsc) => {
        tsc.to_action(&view.ui, &view.scroller).map(|tsa| {
          ActiveTasksViewAction::TasksScroll {
            tsa,
            view: Rc::downgrade(&Rc::clone(view)),
          }
        })
      }

      UndoBuffer(ubc) => {
        let uba = ubc.to_action(&view.undo_buffer);
        Some(ActiveTasksViewAction::UndoBuffer { uba })
      }
    }
  }

  pub fn maybe_add_to_undo_buffer(self, undo_buffer: &UndoBuffer) {
    use self::ActiveTasksViewAction::*;
    match &self {
      Filterer { .. } => undo_buffer.append_action(Box::new(self)),
      Scroll { .. } => return,
      Task { .. } => undo_buffer.append_action(Box::new(self)),
      TasksScroll { .. } => return,
      UndoBuffer { .. } => return,
    }
  }
}

impl ForwardAction for ActiveTasksViewAction {
  fn execute(&mut self) {
    use self::ActiveTasksViewAction::*;
    match self {
      Filterer {
        fa,
        view,
        scroller_state,
      } => {
        execution_logic::execute_filterer_action(
          fa,
          view,
          scroller_state,
        );
      }
      Scroll { sa, .. } => {
        sa.execute();
      }
      Task {
        ta,
        view,
        scroller_state,
      } => {
        execution_logic::execute_task_action(ta, view, scroller_state);
      }
      TasksScroll { tsa, .. } => {
        tsa.execute();
      }
      UndoBuffer { uba } => uba.execute(),
    };
  }
}

impl ReversableAction for ActiveTasksViewAction {
  fn redo(&mut self) {
    use self::ActiveTasksViewAction::*;
    match self {
      Filterer {
        fa,
        view,
        scroller_state,
      } => {
        execution_logic::redo_filterer_action(fa, view, scroller_state);
      }
      Scroll { .. } => {
        panic!("Should not try to redo a Scroll action.");
      }
      Task {
        ta,
        view,
        scroller_state,
      } => {
        execution_logic::redo_task_action(ta, view, scroller_state);
      }
      TasksScroll { .. } => {
        panic!("Should not try to redo a TasksScroll action.");
      }
      UndoBuffer { .. } => {
        panic!("Should not try to redo an UndoBuffer action.")
      }
    };
  }

  fn unexecute(&mut self) {
    use self::ActiveTasksViewAction::*;
    match self {
      Filterer {
        fa,
        view,
        scroller_state,
      } => {
        execution_logic::unexecute_filterer_action(
          fa,
          view,
          scroller_state,
        );
      }
      Scroll { .. } => {
        panic!("Should not try to unexecute a Scroll action.")
      }
      Task {
        ta,
        view,
        scroller_state,
      } => {
        execution_logic::unexecute_task_action(
          ta,
          view,
          scroller_state,
        );
      }
      TasksScroll { .. } => {
        panic!("Should not try to unexecute a TasksScroll action.")
      }
      UndoBuffer { .. } => {
        panic!("Should not try to unexecute an UnderBuffer action.")
      }
    }
  }
}
