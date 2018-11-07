pub trait ReversableAction: ForwardAction {
  fn unexecute(&mut self);
}

pub trait ForwardAction {
  fn execute(&mut self);
}
