DROP TABLE IF EXISTS tank_center;

CREATE TABLE if not exists tank_center (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  tank_id INT NOT NULL,
  key FLOAT8 NOT NULL,
  value_x FLOAT8 NOT NULL,
  value_y FLOAT8 NOT NULL,
  value_z FLOAT8 NOT NULL,
  CONSTRAINT tank_center_pk PRIMARY KEY (id),
  CONSTRAINT tank_center_key_unique UNIQUE (tank_id, key),
  CONSTRAINT tank_center_key_non_negative CHECK (key >= 0)
);

INSERT INTO tank_center
  (project_id, ship_id, tank_id, key, value_x, value_y, value_z)
VALUES
  (NULL, 1, 1, 0, 0, 0, 0),
  (NULL, 1, 1, 10, 1, 1, 1);

SELECT * FROM tank_center WHERE tank_id=1;

TRUNCATE TABLE tank_center;
