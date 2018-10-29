extern crate diesel;
extern crate task_manager;

use diesel::prelude::*;
use task_manager::*;
use task_manager::models::*;

fn main() {
    use task_manager::schema::tasks::dsl::*;

  let connection = establish_connection();

  let results = tasks.load::<Task>(&connection).unwrap();

  for task in results {
    println!("{:?}", task);
  }

  println!("Hello, world!");
}
