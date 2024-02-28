DROP TABLE IF EXISTS center_shift;
CREATE TABLE if not exists center_shift (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key FLOAT8 NOT NULL,
  value_x FLOAT8 NOT NULL,
  value_y FLOAT8 NOT NULL,
  value_z FLOAT8 NOT NULL,
  CONSTRAINT center_shift_unique UNIQUE (ship_id, key),
  CONSTRAINT center_shift_pk PRIMARY KEY (id)
);

INSERT INTO center_shift
  (project_id, ship_id, key, value_x, value_y, value_z)
VALUES
  (NULL, 1, 0, 0, 0, 0),
  (NULL, 1, 10, 1, 1, 1);

SELECT * FROM center_shift WHERE ship_id=1;

SELECT (key, value) FROM center_shift WHERE ship_id=1;