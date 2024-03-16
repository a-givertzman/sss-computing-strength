DROP TABLE IF EXISTS rad_cross;

CREATE TABLE if not exists rad_cross (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key FLOAT8 NOT NULL,
  value FLOAT8 NOT NULL,
  CONSTRAINT rad_cross_pk PRIMARY KEY (id),
  CONSTRAINT rad_cross_unique UNIQUE (ship_id, key)
);

INSERT INTO rad_cross
  (project_id, ship_id, key, value)
VALUES
  (NULL, 1, 0, 0),
  (NULL, 1, 10, 1);

SELECT * FROM rad_cross WHERE ship_id=1;

TRUNCATE TABLE rad_cross;
