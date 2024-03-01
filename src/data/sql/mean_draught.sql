DROP TABLE IF EXISTS mean_draught;

CREATE TABLE if not exists mean_draught (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key REAL NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT mean_draught_pk PRIMARY KEY (id), 
  CONSTRAINT mean_draught_unique UNIQUE (ship_id, key)
);

INSERT INTO mean_draught
  (project_id, ship_id, key, value)
VALUES
  (NULL, 1, 0, 0),
  (NULL, 1, 10, 1);

SELECT * FROM mean_draught WHERE ship_id=1;

SELECT (key, value) FROM mean_draught WHERE ship_id=1;