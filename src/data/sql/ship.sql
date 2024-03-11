DROP TABLE IF EXISTS ship;

CREATE TABLE if not exists ship (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key TEXT NOT NULL,
  value REAL NOT NULL,
  name TEXT NOT NULL,
  unit TEXT,
  CONSTRAINT ship_pk PRIMARY KEY (id),
  CONSTRAINT ship_unique UNIQUE (ship_id, key),
  CONSTRAINT ship_key_check CHECK(char_length(key) <= 50),
  CONSTRAINT ship_name_check CHECK(char_length(name) <= 50),
  CONSTRAINT ship_unit_check CHECK(char_length(unit) <= 10)
);

INSERT INTO ship
  (project_id, ship_id, key, value, name, unit)
VALUES
  (NULL, 1, 'ship_length', 200, 'Ship Length', 'm'),
  (NULL, 1, 'water_density', 1.025, 'Water Density', 'g/ml'),
  (NULL, 1, 'n_parts', 20, 'Number of Parts', NULL),
  (NULL, 1, 'stock', 0, 'Percentage of stocks', '%');

SELECT * FROM ship WHERE ship_id=1;

SELECT key, value FROM ship WHERE ship_id=1;