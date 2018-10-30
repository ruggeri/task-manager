use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get() -> PgConnection {
  let database_url = "postgres://ruggeri@localhost:5432/task_manager";
  PgConnection::establish(&database_url).unwrap_or_else(|_| {
    panic!("Error connecting to {}", database_url);
  })
}
