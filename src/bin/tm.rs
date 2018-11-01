extern crate task_manager;

use task_manager::components::Reviewer;

fn main() {
  Reviewer::new(1000).run();
}
