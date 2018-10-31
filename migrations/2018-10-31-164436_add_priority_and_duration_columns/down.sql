BEGIN TRANSACTION;
  ALTER TABLE
    tasks
  DROP COLUMN
    duration;

  ALTER TABLE
    tasks
  DROP COLUMN
    priority;

  DROP TYPE task_duration;

  DROP TYPE task_priority;

END TRANSACTION;
