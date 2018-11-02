BEGIN TRANSACTION;
  ALTER TABLE
    tasks
  DROP COLUMN
    destroyed;

  ALTER TABLE
    task_efforts
  DROP COLUMN
    destroyed;
END TRANSACTION;
