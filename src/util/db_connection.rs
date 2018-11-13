use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn get_db_connection() -> PgConnection {
  // TODO: Break this out into a configuration file I guess. Look how I
  // don't specify even localhost. By doing so, I will use a Unix domain
  // socket, which means I won't be blocked by my firewall.
  let database_url = "postgres://ruggeri@:5432/task_manager";
  PgConnection::establish(&database_url).unwrap_or_else(|_| {
    panic!("Error connecting to {}", database_url);
  })
}
