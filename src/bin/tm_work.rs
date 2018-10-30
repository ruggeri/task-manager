extern crate task_manager;

use task_manager::reviewer::Reviewer;

fn main() {
  Reviewer::new(3).run();
}
