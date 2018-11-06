use actions::Action;
use application::Application;

#[derive(Clone, Copy, Debug)]
pub enum UndoBufferCommand {
  Redo,
  Undo,
}

impl Action for UndoBufferCommand {
  fn execute(&mut self, application: &Application) {
    use self::UndoBufferCommand::*;

    let undo_buffer = &application.undo_buffer;
    match self {
      Redo => undo_buffer.redo(application),
      Undo => undo_buffer.undo(application),
    }
  }

  fn unexecute(&mut self, _application: &Application) {
    panic!("One does not simply 'undo' an UndoBufferCommand")
  }

  fn can_be_unexecuted(&self) -> bool {
    false
  }
}
