DROP TABLE IF EXISTS center_waterline;

CREATE TABLE if not exists center_waterline (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key REAL NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT center_waterline_pk PRIMARY KEY (id),
  CONSTRAINT center_waterline_unique UNIQUE (ship_id, key)
);

INSERT INTO center_waterline
  (project_id, ship_id, key, value)
VALUES
  (NULL, 1, 0, 0),
  (NULL, 1, 10, 1);

SELECT * FROM center_waterline WHERE ship_id=1;

SELECT (key, value) FROM center_waterline WHERE ship_id=1;