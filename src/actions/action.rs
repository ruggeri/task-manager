pub trait ForwardAction {
  fn execute(&mut self);
}

pub trait ReversableAction: ForwardAction {
  fn redo(&mut self) {
    self.execute();
  }

  fn unexecute(&mut self);
}
