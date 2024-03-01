DROP TABLE IF EXISTS load_space;

CREATE TABLE if not exists load_space (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key TEXT NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT load_space_pk PRIMARY KEY (id),
  CONSTRAINT load_space_key_unique UNIQUE (ship_id, key),
  CONSTRAINT load_space_key_check CHECK(char_length(key) <= 50)
);

INSERT INTO load_space
  (project_id, ship_id, key, value)
VALUES
  (NULL, 1, 'density', 1),
  (NULL, 1, 'volume', 1000),
  (NULL, 1, 'bound_x1', -10),
  (NULL, 1, 'bound_x2', 4),
  (NULL, 1, 'bound_y1', -2),
  (NULL, 1, 'bound_y2', 0),
  (NULL, 1, 'center_x', -2),
  (NULL, 1, 'center_y', -1),
  (NULL, 1, 'center_z', 1);

SELECT * FROM load_space WHERE ship_id=1;

SELECT (id, key, value) FROM load_space WHERE ship_id=1;