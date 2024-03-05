DROP TABLE IF EXISTS tank;

CREATE TABLE if not exists tank (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  tank_id INT NOT NULL,
  key TEXT NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT tank_pk PRIMARY KEY (id),
  CONSTRAINT tank_key_unique UNIQUE (tank_id, key),
  CONSTRAINT tank_key_check CHECK(char_length(key) <= 50)
);

INSERT INTO tank
  (project_id, ship_id, tank_id, key, value)
VALUES
  (NULL, 1, 1, 'density', 1),
  (NULL, 1, 1, 'volume', 1000),
  (NULL, 1, 1, 'bound_x1', -10),
  (NULL, 1, 1, 'bound_x2', 4),
  (NULL, 1, 1, 'bound_y1', -2),
  (NULL, 1, 1, 'bound_y2', 0);

SELECT * FROM tank WHERE ship_id=1;

SELECT tank_id, key, value FROM tank WHERE ship_id=1;