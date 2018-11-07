use actions::ForwardAction;
use commands::UndoBufferCommand;
use components::UndoBuffer;
use std::rc::Rc;

#[derive(Clone)]
pub struct UndoBufferAction {
  cmd: UndoBufferCommand,
  undo_buffer: Rc<UndoBuffer>,
}

impl ForwardAction for UndoBufferAction {
  fn execute(&mut self) {
    use self::UndoBufferCommand::*;

    match self.cmd {
      Redo { undo_buffer } => undo_buffer.redo(),
      Undo { undo_buffer } => undo_buffer.undo(),
    }
  }
}
