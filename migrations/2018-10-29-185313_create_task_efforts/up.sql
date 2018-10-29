CREATE TABLE task_efforts (
  id SERIAL PRIMARY KEY,
  task_id INTEGER REFERENCES tasks (id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL
);
