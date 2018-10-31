BEGIN TRANSACTION;

  CREATE TYPE task_priority AS ENUM (
    'high',
    'medium',
    'low'
  );

  CREATE TYPE task_duration AS ENUM (
    'long',
    'medium',
    'short'
  );

  ALTER TABLE
    tasks
  ADD COLUMN
    priority task_priority NOT NULL DEFAULT 'low';

  ALTER TABLE
    tasks
  ADD COLUMN
    duration task_duration NOT NULL DEFAULT 'short';

END TRANSACTION;
