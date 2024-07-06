-- Add migration script here
CREATE TABLE subscriptions(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  created_AT timestamptz NOT NULL
)