use actions::ReversableAction;
use std::cell::{Cell, RefCell};

pub struct UndoBuffer {
  actions: RefCell<Vec<Box<dyn ReversableAction>>>,
  idx: Cell<Option<usize>>,
}

impl UndoBuffer {
  #![allow(new_without_default_derive)]
  pub fn new() -> UndoBuffer {
    UndoBuffer {
      actions: RefCell::new(vec![]),
      idx: Cell::new(None)
    }
  }

  pub fn redo(&self) {
    let redo_idx = match self.idx.get() {
      None => 0,
      Some(idx) => idx + 1,
    };

    let mut actions = self.actions.borrow_mut();
    if redo_idx >= actions.len() {
      return;
    }

    let result = actions[redo_idx].execute();
    self.idx.set(Some(redo_idx));

    result
  }

  pub fn undo(&self) {
    let idx = match self.idx.get() {
      None => return,
      Some(idx) => idx,
    };

    let mut actions = self.actions.borrow_mut();
    let result = actions[idx].unexecute();
    self.idx.set(if idx > 0 { Some(idx - 1) } else { None });

    result
  }

  pub fn append_action(&self, action: Box<dyn ReversableAction>) {
    let mut actions = self.actions.borrow_mut();

    if let Some(idx) = self.idx.get() {
      // Notice I want to keep the first `idx + 1` actions because I
      // want `idx` to remain a valid index.
      actions.truncate(idx + 1);
    }

    actions.push(action);
    self.idx.set(Some(actions.len() - 1));
  }
}
