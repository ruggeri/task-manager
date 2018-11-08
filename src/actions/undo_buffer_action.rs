use actions::ForwardAction;
use commands::UndoBufferCommand;
use components::UndoBuffer;
use std::rc::Rc;

#[derive(Clone)]
pub struct UndoBufferAction {
  pub cmd: UndoBufferCommand,
  pub undo_buffer: Rc<UndoBuffer>,
}

impl UndoBufferAction {
  pub fn prepare_from_cmd(cmd: UndoBufferCommand, undo_buffer: &Rc<UndoBuffer>) -> UndoBufferAction {
    UndoBufferAction {
      cmd,
      undo_buffer: Rc::clone(undo_buffer),
    }
  }
}

impl ForwardAction for UndoBufferAction {
  fn execute(&mut self) {
    use self::UndoBufferCommand::*;

    match self.cmd {
      Redo => self.undo_buffer.redo(),
      Undo => self.undo_buffer.undo(),
    };
  }
}
