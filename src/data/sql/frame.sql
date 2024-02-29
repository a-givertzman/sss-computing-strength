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

INSERT INTO frame
  (project_id, ship_id, index, delta_x, key, value)
VALUES
  (NULL, 1, 0, -10, 0, 'Ship Length', 'm'),
  (NULL, 1, 'water_density', 1.025, 'Water Density', 'g/ml'),
  (NULL, 1, 'n_parts', 20, 'Number of Parts', NULL);

SELECT * FROM ship_parameters WHERE ship_id=1;

SELECT (key, value) FROM ship_parameters WHERE ship_id=1;