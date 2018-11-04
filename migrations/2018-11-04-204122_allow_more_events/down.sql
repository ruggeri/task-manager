BEGIN TRANSACTION;
  ALTER TABLE
    task_events
  DROP COLUMN
    event_type;

  ALTER TABLE
    task_events
  RENAME TO
    task_efforts;

  DROP TYPE task_event_type;
END TRANSACTION;

