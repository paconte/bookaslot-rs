-- Your SQL goes here
CREATE TABLE bookable (
  id integer PRIMARY KEY,
  name varchar NOT NULL
);

CREATE TABLE slots (
  id integer PRIMARY KEY,
  state varchar NOT NULL,
  start bigint NOT NULL,
  finish bigint NOT NULL,
  bookable integer REFERENCES bookable (id) ON DELETE RESTRICT NOT NULL
);