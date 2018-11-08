use actions::scroller_state::SavedScrolerState;
use actions::{FiltererAction, ScrollAction, TaskAction, UndoBufferAction};
use commands::ActiveTasksViewCommand;
use components::UndoBuffer;
use std::rc::{Rc, Weak};
use views::ActiveTasksView;

#[derive(Clone)]
pub enum ActiveTasksViewAction {
  Filterer {
    fa: FiltererAction,
    view: Weak<ActiveTasksView>,
    scroller_state: SavedScrolerState,
  },
  Scroll {
    sa: ScrollAction,
    view: Weak<ActiveTasksView>,
  },
  Task {
    ta: TaskAction,
    view: Weak<ActiveTasksView>,
    scroller_state: SavedScrolerState,
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
        fc.to_action(&view.ui, &view.filterer)
          .map(|fa| ActiveTasksViewAction::Filterer {
            fa,
            view: Rc::downgrade(&view),
            scroller_state: SavedScrolerState::new(&view.scroller),
          })
      }

      Scroll(sc) => {
        sc.to_action(&view.ui, &view.scroller)
          .map(|sa| ActiveTasksViewAction::Scroll {
            sa,
            view: Rc::downgrade(&Rc::clone(view)),
          })
      }

      Task(tc) => tc
        .to_action(&view.ui, &view.connection, || view.scroller.current_task())
        .map(|ta| ActiveTasksViewAction::Task {
          ta,
          view: Rc::downgrade(&Rc::clone(view)),
          scroller_state: SavedScrolerState::new(&view.scroller),
        }),

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
      UndoBuffer { .. } => return,
    }
  }
}
