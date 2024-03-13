DROP TABLE IF EXISTS rad_long;

CREATE TABLE if not exists rad_long (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key FLOAT8 NOT NULL,
  value FLOAT8 NOT NULL,
  CONSTRAINT rad_long_pk PRIMARY KEY (id),
  CONSTRAINT rad_long_unique UNIQUE (ship_id, key)
);

INSERT INTO rad_long
  (project_id, ship_id, key, value)
VALUES
  (NULL, 1, 0, 0),
  (NULL, 1, 10, 1);

SELECT * FROM rad_long WHERE ship_id=1;

TRUNCATE TABLE rad_long;