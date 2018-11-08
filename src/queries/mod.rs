macro_rules! define_update_attribute_fn {
  ($table:ident, $fn_name:ident, $value_type:ty, $field_name:ident) => {
    pub fn $fn_name(id: i32, new_value: $value_type, connection: &PgConnection) {
      use schema::$table::dsl;

      let num_updated = diesel::update(dsl::$table.find(id))
        .set(dsl::$field_name.eq(new_value))
        .execute(connection)
        .expect("Error updating task");

      if num_updated != 1 {
        panic!("Expected to update exactly one task");
      }
    }
  };
}

macro_rules! define_update_attribute_fns {
  ($table:ident, $(($fn_name:ident, $value_type:ty, $field_name:ident)),*) => {
    $(
      define_update_attribute_fn!($table, $fn_name, $value_type, $field_name);
    )+
  }
}

pub mod task;
pub mod task_event;
