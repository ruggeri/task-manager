use super::saved_scroller_state::{
  NewScrollerTaskId, SavedTasksScrolerState,
};
use actions::{
  FiltererAction, ForwardAction, ReversableAction, TaskAction,
  TaskUpdateAction,
};
use components::{Scroller, TasksScroller};
use models::End;
use std::rc::Weak;
use views::ActiveTasksView;

fn jump_to_task_id_or_top(scroller: &TasksScroller, task_id: i32) {
  if !scroller.jump_to_task_id(task_id) {
    // Sometimes a task can sneak away. In that case just jump to top.
    scroller.jump(End::Top);
  }
}

fn jump_to_task_id_option_or_top(
  scroller: &TasksScroller,
  task_id_option: Option<i32>,
) {
  match task_id_option {
    None => scroller.jump(End::Top),
    Some(task_id) => jump_to_task_id_or_top(scroller, task_id),
  }
}

// == EXECUTE CODE ==

pub fn execute_filterer_action(
  fa: &mut FiltererAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedTasksScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.old_id = view.scroller.current_task_id();

  // Now execute filtering action.
  fa.execute();

  // Fetch new data when filterer is applied.
  view.data_source.pull(&view.connection);
  view.scroller.jump(End::Top);
}

pub fn execute_task_action(
  ta: &mut TaskAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedTasksScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.old_id = view.scroller.current_task_id();
  let old_result_idx = view.scroller.current_result_idx();

  // Now execute task action.
  ta.execute();

  // Fetch data when any task is updated.
  view.data_source.pull(&view.connection);

  use self::TaskAction::*;
  use self::TaskUpdateAction::*;
  match ta {
    CreateTask { task, .. } => {
      let task = task.as_ref().expect("Task should have been created.");
      jump_to_task_id_or_top(&view.scroller, task.id);
    }

    RecordTaskEffort { .. } | RequestTaskDelay { .. } => {
      // First time, just try to stay at the idx you are at.
      view.scroller.set_current_result_idx(old_result_idx);
    }

    TaskUpdate(UpdateDuration { task_id, .. })
    | TaskUpdate(UpdatePriority { task_id, .. })
    | TaskUpdate(UpdateRequiresInternet { task_id, .. })
    | TaskUpdate(UpdateStatus { task_id, .. })
    | TaskUpdate(UpdateTaskTitle { task_id, .. }) => {
      // Try to follow task
      if !view.scroller.jump_to_task_id(*task_id) {
        // Maybe can't follow because for instance changed status and
        // was removed from results. First time, just try to stay at the
        // idx you are at.
      }
    }
  }
}

// == REDO CODE ==

pub fn redo_filterer_action(
  fa: &mut FiltererAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedTasksScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.old_id = view.scroller.current_task_id();

  // Now execute filtering action.
  fa.execute();

  // Fetch new data when filterer is applied.
  view.data_source.pull(&view.connection);

  // Try to restore new scroller state.
  let new_task_id = scroller_state.unwrap_new_id();
  jump_to_task_id_option_or_top(&view.scroller, new_task_id);
}

pub fn redo_task_action(
  ta: &mut TaskAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedTasksScrolerState,
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
      jump_to_task_id_or_top(&view.scroller, task.id);
    }

    RecordTaskEffort { .. } | RequestTaskDelay { .. } => {
      // Try to restore id that had been focused on.
      let new_task_id = scroller_state.unwrap_new_id();
      jump_to_task_id_option_or_top(&view.scroller, new_task_id);
    }

    TaskUpdate(UpdateDuration { task_id, .. })
    | TaskUpdate(UpdatePriority { task_id, .. })
    | TaskUpdate(UpdateRequiresInternet { task_id, .. })
    | TaskUpdate(UpdateStatus { task_id, .. })
    | TaskUpdate(UpdateTaskTitle { task_id, .. }) => {
      // Try to follow task forward.
      if !view.scroller.jump_to_task_id(*task_id) {
        // Task may have been been removed, in which case try to focus
        // on last selected index.
        let new_id = scroller_state.unwrap_new_id();
        jump_to_task_id_option_or_top(&view.scroller, new_id);
      }
    }
  }
}

// == UNDO CODE ==

pub fn unexecute_filterer_action(
  fa: &mut FiltererAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedTasksScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.new_id =
    NewScrollerTaskId::Saved(view.scroller.current_task_id());

  // Now execute filtering action.
  fa.unexecute();

  // Fetch new data when filterer is applied.
  view.data_source.pull(&view.connection);

  // Try to restore scroll position.
  jump_to_task_id_option_or_top(&view.scroller, scroller_state.old_id);
}

pub fn unexecute_task_action(
  ta: &mut TaskAction,
  view: &Weak<ActiveTasksView>,
  scroller_state: &mut SavedTasksScrolerState,
) {
  let view = view.upgrade().expect("Action should not outlive view");

  // First save scroller position.
  scroller_state.new_id =
    NewScrollerTaskId::Saved(view.scroller.current_task_id());

  // Now unexecute task action.
  ta.unexecute();

  // Fetch data when any task is updated.
  view.data_source.pull(&view.connection);

  use self::TaskAction::*;
  use self::TaskUpdateAction::*;
  match ta {
    CreateTask { .. } => {
      // Try to return to previously focused task.
      jump_to_task_id_option_or_top(
        &view.scroller,
        scroller_state.old_id,
      );
    }

    RecordTaskEffort { task_id, .. }
    | RequestTaskDelay { task_id, .. }
    | TaskUpdate(UpdateDuration { task_id, .. })
    | TaskUpdate(UpdatePriority { task_id, .. })
    | TaskUpdate(UpdateRequiresInternet { task_id, .. })
    | TaskUpdate(UpdateStatus { task_id, .. })
    | TaskUpdate(UpdateTaskTitle { task_id, .. }) => {
      // Try to follow task id back.
      jump_to_task_id_or_top(&view.scroller, *task_id);
    }
  }
}
