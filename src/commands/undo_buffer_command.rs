use actions::Action;
use components::Reviewer;

#[derive(Clone, Copy, Debug)]
pub enum UndoBufferCommand {
  Redo,
  Undo,
}

impl Action for UndoBufferCommand {
  fn execute(&mut self, reviewer: &Reviewer) {
    use self::UndoBufferCommand::*;

    let undo_buffer = &reviewer.undo_buffer;
    match self {
      Redo => undo_buffer.redo(reviewer),
      Undo => undo_buffer.undo(reviewer),
    }
  }

  fn unexecute(&mut self, _reviewer: &Reviewer) {
    panic!("One does not simply 'undo' an UndoBufferCommand")
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
