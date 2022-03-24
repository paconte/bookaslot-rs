-- Your SQL goes here
CREATE TABLE bookable (
  id serial PRIMARY KEY,
  name varchar NOT NULL
);

CREATE TABLE slots (
  id serial PRIMARY KEY,
  state varchar NOT NULL,
  start bigint NOT NULL,
  finish bigint NOT NULL,
  bookable_id integer REFERENCES bookable (id) ON DELETE RESTRICT NOT NULL
);