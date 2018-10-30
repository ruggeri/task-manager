use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get() -> PgConnection {
  let database_url = "postgres://ruggeri@localhost:5432/task_manager";
  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
