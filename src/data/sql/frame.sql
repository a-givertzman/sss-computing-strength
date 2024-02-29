DROP TABLE IF EXISTS frame;

CREATE TABLE if not exists frame (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  index INT NOT NULL,,
  delta_x FLOAT8 NOT NULL,
  key FLOAT8 NOT NULL,
  value FLOAT8 NOT NULL,
  CONSTRAINT frame_index_unique UNIQUE (ship_id, index),
  CONSTRAINT frame_index_non_negative UNIQUE (index >= 0),
  CONSTRAINT frame_delta_x_unique UNIQUE (ship_id, delta_x),
  CONSTRAINT frame_key_unique UNIQUE (ship_id, key),
  CONSTRAINT frame_key_non_negative UNIQUE (key >= 0),
  CONSTRAINT frame_pk PRIMARY KEY (id)
);
