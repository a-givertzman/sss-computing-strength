DROP TABLE IF EXISTS tank_center;

CREATE TABLE if not exists tank_center (
  id INT GENERATED ALWAYS AS IDENTITY,
  tank_id INT NOT NULL,
  key REAL NOT NULL,
  value_x REAL NOT NULL,
  value_y REAL NOT NULL,
  value_z REAL NOT NULL,
  CONSTRAINT tank_center_pk PRIMARY KEY (id),
  CONSTRAINT tank_center_key_unique UNIQUE (tank_id, key),
  CONSTRAINT tank_center_key_non_negative CHECK (key >= 0)
);

INSERT INTO tank_center
  (tank_id, key, value_x, value_y, value_z)
VALUES
  (1, 0, 0, 0, 0),
  (1, 10, 1, 1, 1);

SELECT * FROM tank_center WHERE tank_id=1;

SELECT (key, value) FROM tank_center WHERE tank_id=1;
