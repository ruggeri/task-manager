BEGIN TRANSACTION;
  CREATE TYPE task_event_type AS ENUM (
    'TaskEffortRecorded',
    'DelayRequested'
  );

  ALTER TABLE
    task_efforts
  RENAME TO
    task_events;

  ALTER TABLE
    task_events
  ADD COLUMN
    event_type task_event_type NOT NULL DEFAULT 'DelayRequested';

  ALTER TABLE
    task_events
  ALTER COLUMN
    event_type DROP DEFAULT;
END TRANSACTION;
