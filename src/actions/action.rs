pub trait ForwardAction {
  fn execute(&mut self);
}

pub trait ReversableAction: ForwardAction {
  fn unexecute(&mut self);
}
