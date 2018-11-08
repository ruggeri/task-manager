use actions::scroller_state::SavedScrolerState;
use actions::{
  ActiveTasksViewAction, ForwardAction, ReversableAction, TaskAction, TaskUpdateAction,
};
use components::Scroller;
use models::End;
use std::rc::Weak;
use views::ActiveTasksView;

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
