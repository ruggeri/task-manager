use actions::{
  FiltererAction, ForwardAction, ReversableAction, ScrollAction, TaskAction, TaskUpdateAction,
  UndoBufferAction,
};
use commands::ActiveTasksViewCommand;
use components::{Scroller, UndoBuffer};
use models::End;
use std::rc::{Rc, Weak};
use views::ActiveTasksView;

#[derive(Clone)]
pub struct SavedScrolerState {
  old_id: Option<i32>,
  new_id: Option<Option<i32>>,
}

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
            view: Rc::downgrade(&Rc::clone(view)),
            scroller_state: SavedScrolerState {
              old_id: view.scroller.current_task_id(),
              new_id: None,
            },
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
          scroller_state: SavedScrolerState {
            old_id: view.scroller.current_task_id(),
            new_id: None,
          },
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

fn maybe_jump_to_task(scroller: &Scroller, task_id: Option<i32>) -> bool {
  task_id.map_or(false, |task_id| scroller.jump_to_task_id(task_id))
}

fn execute_task_action(
  ta: &mut TaskAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.old_id = view.scroller.current_task_id();

  // Now execute task action.
  ta.execute();

  // Fetch data when any task is updated.
  view.data_source.pull(&view.connection);

  use self::TaskAction::*;
  use self::TaskUpdateAction::*;
  match ta {
    CreateTask { task, .. } => {
      let task = task.as_ref().expect("Task should have been created.");
      if !view.scroller.jump_to_task_id(task.id) {
        // Weird. Where did the task go?
        view.scroller.jump(End::Top);
      }
    }
    RecordTaskEffort { .. } | RequestTaskDelay { .. } => {
      // First time, just try to stay at the idx you are at.
    }
    TaskUpdate(UpdateDuration { task_id, .. })
    | TaskUpdate(UpdatePriority { task_id, .. })
    | TaskUpdate(UpdateRequiresInternet { task_id, .. })
    | TaskUpdate(UpdateStatus { task_id, .. })
    | TaskUpdate(UpdateTaskTitle { task_id, .. }) => {
      // Try to follow task
      if !view.scroller.jump_to_task_id(*task_id) {
        // First time, just try to stay at the idx you are at.
      }
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
        let view = view.upgrade().expect("Action should not outlive view");

        // First save scroller position.
        scroller_state.old_id = view.scroller.current_task_id();

        // Now execute filtering action.
        fa.execute();

        // Fetch new data when filterer is applied.
        view.data_source.pull(&view.connection);
        view.scroller.jump(End::Top);
      }
      Scroll { sa, .. } => {
        sa.execute();
      }
      Task {
        ta,
        view,
        scroller_state,
      } => {
        execute_task_action(ta, view, scroller_state);
      }
      UndoBuffer { uba } => uba.execute(),
    };
  }
}

fn redo_task_action(
  ta: &mut TaskAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.old_id = view.scroller.current_task_id();

  // Now execute task action.
  ta.execute();

  // Fetch data when any task is updated.
  view.data_source.pull(&view.connection);

  use self::TaskAction::*;
  use self::TaskUpdateAction::*;
  match ta {
    CreateTask { task, .. } => {
      let task = task.as_ref().expect("Task should have been created.");
      if !view.scroller.jump_to_task_id(task.id) {
        // Weird. Where did the task go?
        view.scroller.jump(End::Top);
      }
    }
    RecordTaskEffort { .. } | RequestTaskDelay { .. } => {
      // Try to restore id that had been focused on.
      if !maybe_jump_to_task(&view.scroller, scroller_state.new_id.unwrap()) {
        view.scroller.jump(End::Top);
      }
    }
    TaskUpdate(UpdateDuration { task_id, .. })
    | TaskUpdate(UpdatePriority { task_id, .. })
    | TaskUpdate(UpdateRequiresInternet { task_id, .. })
    | TaskUpdate(UpdateStatus { task_id, .. })
    | TaskUpdate(UpdateTaskTitle { task_id, .. }) => {
      // Try to follow task, but if you can't, then try to restore
      // cursor.
      let did_update_scroller =
        view.scroller.jump_to_task_id(*task_id)
          || maybe_jump_to_task(&view.scroller, scroller_state.new_id.unwrap());

      if !did_update_scroller {
        view.scroller.jump(End::Top);
      }
    }
  }
}

fn unexecute_task_action(
  ta: &mut TaskAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.new_id = Some(view.scroller.current_task_id());

  // Now unexecute task action.
  ta.unexecute();

  // Fetch data when any task is updated.
  view.data_source.pull(&view.connection);

  use self::TaskAction::*;
  use self::TaskUpdateAction::*;
  match ta {
    CreateTask { .. } => {
      // Try to return to old task id.
      if !maybe_jump_to_task(&view.scroller, scroller_state.old_id) {
        // But if can't then jump to top.
        view.scroller.jump(End::Top);
      }
    }
    RecordTaskEffort { task_id, .. }
    | RequestTaskDelay { task_id, .. }
    | TaskUpdate(UpdateDuration { task_id, .. })
    | TaskUpdate(UpdatePriority { task_id, .. })
    | TaskUpdate(UpdateRequiresInternet { task_id, .. })
    | TaskUpdate(UpdateStatus { task_id, .. })
    | TaskUpdate(UpdateTaskTitle { task_id, .. }) => {
      // Try to follow task_id back.
      if !view.scroller.jump_to_task_id(*task_id) {
        // But if can't then jump to top.
        view.scroller.jump(End::Top);
      }
    }
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
        let view = view.upgrade().expect("Action should not outlive view");

        // First save scroller position.
        scroller_state.old_id = view.scroller.current_task_id();

        // Now execute filtering action.
        fa.execute();

        // Fetch new data when filterer is applied.
        view.data_source.pull(&view.connection);

        // Try to restore new scroller state.
        if !maybe_jump_to_task(&view.scroller, scroller_state.new_id.unwrap()) {
          view.scroller.jump(End::Top);
        }
      }
      Scroll { .. } => {
        panic!("Should not try to redo a Scroll action.");
      }
      Task {
        ta,
        view,
        scroller_state,
      } => {
        redo_task_action(ta, view, scroller_state);
      }
      UndoBuffer { .. } => panic!("Should not try to redo an UndoBuffer action."),
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
        let view = view.upgrade().expect("Action should not outlive view");

        // First save scroller position.
        scroller_state.new_id = Some(view.scroller.current_task_id());

        // Now execute filtering action.
        fa.unexecute();

        // Fetch new data when filterer is applied.
        view.data_source.pull(&view.connection);

        // Restore scroller.
        let did_jump_to_task_id = scroller_state.old_id.map_or(false, |old_task_id| {
          view.scroller.jump_to_task_id(old_task_id)
        });

        // If couldn't jump to a task by id, then just jump to top.
        if !did_jump_to_task_id {
          view.scroller.jump(End::Top);
        }
      }
      Scroll { .. } => panic!("Should not try to unexecute a Scroll action."),
      Task {
        ta,
        view,
        scroller_state,
      } => {
        unexecute_task_action(ta, view, scroller_state);
      }
      UndoBuffer { .. } => panic!("Should not try to unexecute an UnderBuffer action."),
    }
  }
}
