use actions::ReversableAction;
use std::cell::{Cell, RefCell};

pub struct UndoBuffer {
  actions: RefCell<Vec<Box<dyn ReversableAction>>>,
  idx: Cell<Option<usize>>,
}

impl UndoBuffer {
  pub fn new() -> UndoBuffer {
    UndoBuffer {
      actions: RefCell::new(vec![]),
      idx: Cell::new(None),
    }
  }

  pub fn redo(&self) {
    let redo_idx = match self.idx.get() {
      None => 0,
      Some(idx) => idx + 1,
    };

    {
      let mut actions = self.actions.borrow_mut();
      if redo_idx >= actions.len() {
        return;
      }

      let action = &mut actions[redo_idx];
      action.redo();
    }

    self.idx.set(Some(redo_idx));
  }

  pub fn undo(&self) {
    let undo_idx = match self.idx.get() {
      None => return,
      Some(idx) => idx,
    };

    {
      let mut actions = self.actions.borrow_mut();
      let action = &mut actions[undo_idx];
      action.unexecute();
    }

    self.idx.set(if undo_idx > 0 {
      Some(undo_idx - 1)
    } else {
      None
    });
  }

  pub fn append_action(&self, action: Box<dyn ReversableAction>) {
    let mut actions = self.actions.borrow_mut();
    if let Some(idx) = self.idx.get() {
      // Notice I want to keep the first `idx + 1` actions because I
      // want `idx` to remain a valid index.
      actions.truncate(idx + 1);
    } else {
      actions.truncate(0);
    }

    actions.push(action);
    self.idx.set(Some(actions.len() - 1));
  }
}

impl Default for UndoBuffer {
  fn default() -> UndoBuffer {
    UndoBuffer::new()
  }
}
