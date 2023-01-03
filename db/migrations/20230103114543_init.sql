-- Add migration script here
CREATE TABLE tasks
(
	id        SERIAL PRIMARY KEY,
	text      TEXT NOT NULL,
	completed BOOLEAN NOT NULL DEFAULT false
);
