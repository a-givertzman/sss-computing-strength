DROP TABLE IF EXISTS ship_parameters;

CREATE TABLE if not exists ship_parameters (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key TEXT NOT NULL,
  value REAL NOT NULL,
  name TEXT NOT NULL,
  unit TEXT,
  CONSTRAINT ship_parameters_pk PRIMARY KEY (id),
  CONSTRAINT ship_parameters_unique UNIQUE (ship_id, key),
  CONSTRAINT ship_parameters_key_check CHECK(char_length(key) <= 50),
  CONSTRAINT ship_parameters_name_check CHECK(char_length(name) <= 50),
  CONSTRAINT ship_parameters_unit_check CHECK(char_length(unit) <= 10)
);

INSERT INTO ship_parameters
  (project_id, ship_id, key, value, name, unit)
VALUES
  (NULL, 1, 'ship_length', 200, 'Ship Length', 'm'),
  (NULL, 1, 'water_density', 1.025, 'Water Density', 'g/ml'),
  (NULL, 1, 'n_parts', 20, 'Number of Parts', NULL);

SELECT * FROM ship_parameters WHERE ship_id=1;

SELECT (key, value) FROM ship_parameters WHERE ship_id=1;