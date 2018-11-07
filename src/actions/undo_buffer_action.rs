use actions::ForwardAction;
use commands::UndoBufferCommand;
use components::UndoBuffer;
use std::rc::Rc;

#[derive(Clone)]
pub struct UndoBufferAction<State> {
  pub cmd: UndoBufferCommand,
  pub undo_buffer: Rc<UndoBuffer<State>>,
}

impl<State> ForwardAction for UndoBufferAction<State> {
  fn execute(&mut self) {
    use self::UndoBufferCommand::*;

    match self.cmd {
      Redo => self.undo_buffer.redo(),
      Undo => self.undo_buffer.undo(),
    };
  }
}
