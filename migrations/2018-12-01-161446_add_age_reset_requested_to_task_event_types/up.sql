-- This is insane. In Postgres you can't alter an enum's values inside a
-- transaction. Otherwise this command should be wrapped.
BEGIN TRANSACTION;
  -- Create new version of task_event_type
  CREATE TYPE task_event_type_new AS ENUM (
    'task_effort_recorded',
    'delay_requested',
    'age_reset_requested'
  );

  -- Add it as a new column with a default value.
  ALTER TABLE
    task_events
  ADD COLUMN
    event_type_new task_event_type_new NOT NULL DEFAULT 'task_effort_recorded';

  -- Copy over all values.
  UPDATE
    task_events
  SET
    event_type_new = CASE
      WHEN event_type = 'task_effort_recorded' THEN
        'task_effort_recorded'::task_event_type_new
      WHEN event_type = 'delay_requested' THEN
        'delay_requested'::task_event_type_new
      ELSE
        -- This should never happen.
        NULL
      END;

  -- Drop the default.
  ALTER TABLE
    task_events
  ALTER COLUMN
    event_type_new DROP DEFAULT;

  -- Drop the old column.
  ALTER TABLE
    task_events
  DROP COLUMN
    event_type;

  -- Rename the new column.
  ALTER TABLE
    task_events
  RENAME COLUMN
    event_type_new TO event_type;

  -- Drop the old type.
  DROP TYPE
    task_event_type;

  -- Rename the new type.
  ALTER TYPE
    task_event_type_new RENAME TO task_event_type;

END TRANSACTION;
