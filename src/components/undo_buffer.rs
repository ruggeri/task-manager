use actions::ReversableAction;
use std::cell::{Cell, RefCell};

type Callback<State> = Fn(&State) -> ();

pub struct UndoItem<State> {
  action: Box<dyn ReversableAction>,
  state_after_action: State,
}

pub struct UndoBuffer<State> {
  initial_state: RefCell<State>,
  items: RefCell<Vec<UndoItem<State>>>,
  idx: Cell<Option<usize>>,
  callback: RefCell<Option<Box<Callback<State>>>>,
}

// TODO: Just need to pass in a method to perform the state update...
impl<State> UndoBuffer<State> {
  #![allow(new_without_default_derive)]
  pub fn new(initial_state: State) -> UndoBuffer<State> {
    UndoBuffer {
      initial_state: RefCell::new(initial_state),
      items: RefCell::new(vec![]),
      idx: Cell::new(None),
      callback: RefCell::new(None),
    }
  }

  pub fn set_callback(&self, callback: Box<Callback<State>>) {
    *self.callback.borrow_mut() = Some(callback)
  }

  pub fn redo(&self) {
    let redo_idx = match self.idx.get() {
      None => 0,
      Some(idx) => idx + 1,
    };

    {
      let mut items = self.items.borrow_mut();
      if redo_idx >= items.len() {
        return;
      }

      let action = &mut items[redo_idx].action;
      action.execute();
    }

    self.idx.set(Some(redo_idx));
    self.execute_callback_on_current_state();
  }

  pub fn undo(&self) {
    let undo_idx = match self.idx.get() {
      None => return,
      Some(idx) => idx,
    };

    {
      let mut items = self.items.borrow_mut();
      let action = &mut items[undo_idx].action;
      action.unexecute();
    }

    self.idx.set(if undo_idx > 0 { Some(undo_idx - 1) } else { None });
    self.execute_callback_on_current_state();
  }

  pub fn execute_callback_on_current_state(&self) {
    let initial_state = self.initial_state.borrow();
    let items = self.items.borrow();
    let current_state = match self.idx.get() {
      None => &initial_state,
      Some(idx) => &items[idx].state_after_action
    };

    let callback_option = self.callback.borrow();
    let callback = callback_option.as_ref().expect("UndoBuffer expects callback");
    callback(current_state);
  }

  pub fn set_current_state(&self, state: State) {
    match self.idx.get() {
      None => { self.initial_state.replace(state); },
      Some(idx) => {
        let mut items = self.items.borrow_mut();
        items[idx].state_after_action = state;
      }
    };
  }

  pub fn append_item(&self, state_after_action: State, action: Box<dyn ReversableAction>) {
    let item = UndoItem {
      action,
      state_after_action,
    };

    let mut items = self.items.borrow_mut();
    if let Some(idx) = self.idx.get() {
      // Notice I want to keep the first `idx + 1` actions because I
      // want `idx` to remain a valid index.
      items.truncate(idx + 1);
    }

    items.push(item);
    self.idx.set(Some(items.len() - 1));
  }
}
