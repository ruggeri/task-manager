use actions::{ForwardAction, ReversableAction};
use commands::UndoBufferCommand;
use components::UndoBuffer;
use std::rc::Rc;

#[derive(Clone)]
pub struct UndoBufferAction<ActionType, State>
where
  ActionType: ReversableAction,
{
  pub cmd: UndoBufferCommand,
  pub undo_buffer: Rc<UndoBuffer<ActionType, State>>,
}

impl<ActionType, State> ForwardAction for UndoBufferAction<ActionType, State>
where
  ActionType: ReversableAction,
{
  fn execute(&mut self) {
    use self::UndoBufferCommand::*;

    match self.cmd {
      Redo => self.undo_buffer.redo(),
      Undo => self.undo_buffer.undo(),
    };
  }
}
