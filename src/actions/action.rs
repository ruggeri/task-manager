use application::Application;

pub trait Action {
  fn execute(&mut self, application: &Application);
  fn unexecute(&mut self, application: &Application);
  fn can_be_unexecuted(&self) -> bool;
}
