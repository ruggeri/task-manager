use actions::ReversableAction;
use std::cell::{Cell, RefCell};

type Callback<State> = Fn(&State) -> ();

pub struct UndoItem<State> {
  action: Box<dyn ReversableAction>,
  state: State,
}

pub struct UndoBuffer<State> {
  items: RefCell<Vec<UndoItem<State>>>,
  idx: Cell<Option<usize>>,
  callback: RefCell<Option<Box<Callback<State>>>>,
}

// TODO: Just need to pass in a method to perform the state update...
impl<State> UndoBuffer<State> {
  #![allow(new_without_default_derive)]
  pub fn new() -> UndoBuffer<State> {
    UndoBuffer {
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

    {
      let items = self.items.borrow();
      let state = &items[redo_idx].state;
      let callback_option = self.callback.borrow();
      let callback = callback_option.as_ref().expect("UndoBuffer expects callback");
      callback(state);
    }
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

    {
      let items = self.items.borrow();
      let state = &items[undo_idx].state;
      let callback_option = self.callback.borrow();
      let callback = callback_option.as_ref().expect("UndoBuffer expects callback");
      callback(state);
    }
  }

  pub fn append_item(&self, state: State, action: Box<dyn ReversableAction>) {
    let item = UndoItem {
      action,
      state,
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
