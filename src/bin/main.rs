extern crate diesel;
extern crate task_manager;

use task_manager::reviewer::Reviewer;

fn main() {
  Reviewer::new().run();
}
