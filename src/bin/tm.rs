extern crate task_manager;

use task_manager::application::Application;

fn main() {
  Application::new(1000).run();
}
