// Because Queryable triggers a Rust lang warning. They'll fix this in a
// future release.
#![allow(proc_macro_derive_resolution_fallback)]

#[derive(DbEnum, Debug)]
pub enum TaskStatus {
  Abandoned,
  AvailableToPerform,
  Completed
}

// pub struct ParseError();

// impl FromStr for TaskStatus {
//   type Err = ParseError;

//   fn from_str(s: &str) -> Result<Self, Self::Err> {
//     use self::TaskStatus::*;

//     let value = match s {
//       "ABANDONED" => Abandoned,
//       "AVAILABLE_TO_PERFORM" => AvailableToPerform,
//       "COMPLETED" => Completed,
//       _ => return Err(ParseError())
//     };

//     Ok(value)
//   }
// }

#[derive(Debug, Queryable)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub status: TaskStatus,
  pub created_at: ::chrono::DateTime<::chrono::Utc>,
}
