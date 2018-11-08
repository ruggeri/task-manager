use actions::{ReversableAction, UndoBufferAction};
use components::UndoBuffer;
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub enum UndoBufferCommand {
  Redo,
  Undo,
}

impl UndoBufferCommand {
  pub fn to_action<ActionType, State>(self, undo_buffer: &Rc<UndoBuffer<ActionType, State>>) -> UndoBufferAction<ActionType, State>
    where ActionType: ReversableAction {
    UndoBufferAction {
      cmd: self,
      undo_buffer: Rc::clone(undo_buffer),
    }
  }
}
