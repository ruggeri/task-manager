use actions::UndoBufferAction;
use components::UndoBuffer;
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub enum UndoBufferCommand {
  Redo,
  Undo,
}

impl UndoBufferCommand {
  pub fn to_action<State>(self, undo_buffer: &Rc<UndoBuffer<State>>) -> UndoBufferAction<State> {
    UndoBufferAction {
      cmd: self,
      undo_buffer: Rc::clone(undo_buffer),
    }
  }
}
