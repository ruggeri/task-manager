use actions::Action;
use components::Reviewer;

pub struct UndoBuffer {
  actions: Vec<Box<dyn Action>>,
  idx: Option<usize>,
}

impl UndoBuffer {
  pub fn new() -> UndoBuffer {
    UndoBuffer {
      actions: vec![],
      idx: None
    }
  }

  pub fn redo(&mut self, reviewer: &mut Reviewer) {
    let redo_idx = match self.idx {
      None => 0,
      Some(idx) => idx + 1,
    };

    if redo_idx >= self.actions.len() {
      return;
    }

    self.actions[redo_idx].execute(reviewer);
    self.idx = Some(redo_idx);
  }

  pub fn undo(&mut self, reviewer: &mut Reviewer) {
    let idx = match self.idx {
      None => return,
      Some(idx) => idx,
    };

    self.actions[idx].unexecute(reviewer);
    self.idx = if idx > 0 { Some(idx - 1) } else { None };
  }

  pub fn append_action(&mut self, action: Box<dyn Action>) {
    if let Some(idx) = self.idx {
      self.actions.truncate(idx);
    }

    self.actions.push(action);
    self.idx = Some(self.actions.len() - 1);
  }
}
