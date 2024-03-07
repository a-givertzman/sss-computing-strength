DROP TABLE IF EXISTS center_draught;

CREATE TABLE if not exists center_draught (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key REAL NOT NULL,
  value_x REAL NOT NULL,
  value_y REAL NOT NULL,
  value_z REAL NOT NULL,
  CONSTRAINT center_draught_pk PRIMARY KEY (id),
  CONSTRAINT center_draught_unique UNIQUE (ship_id, key)
);

INSERT INTO center_draught
  (project_id, ship_id, key, value_x, value_y, value_z)
VALUES
  (NULL, 1, 0, 0, 0, 0),
  (NULL, 1, 10, 1, 1, 1);

SELECT * FROM center_draught WHERE ship_id=1;

SELECT key, value FROM center_draught WHERE ship_id=1;