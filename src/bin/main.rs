extern crate diesel;
extern crate task_manager;

use diesel::prelude::*;
use task_manager::{models::*, reviewer::Reviewer};

fn main() {
  // I believe this brings the DSL traits into scope...
  use task_manager::schema::tasks::dsl::*;

  let connection = task_manager::establish_connection();
  let results = tasks.load::<Task>(&connection).unwrap();
  let reviewer = Reviewer::new(results);
  reviewer.run();
}
