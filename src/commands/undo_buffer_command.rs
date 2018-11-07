use actions::UndoBufferAction;
use components::UndoBuffer;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum UndoBufferCommand {
  Redo,
  Undo,
}

impl UndoBufferCommand {
  pub fn to_action(self, undo_buffer: &Rc<UndoBuffer>) -> UndoBufferAction {
    UndoBufferAction {
      cmd: self,
      undo_buffer_command: Rc::clone(undo_buffer),
    }
  }
}
