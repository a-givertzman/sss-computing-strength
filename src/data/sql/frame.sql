DROP TABLE IF EXISTS frame;

CREATE TABLE if not exists frame (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  index INT NOT NULL,
  delta_x REAL NOT NULL,
  CONSTRAINT frame_pk PRIMARY KEY (id),
  CONSTRAINT frame_index_unique UNIQUE (ship_id, index),
  CONSTRAINT frame_index_non_negative CHECK (index >= 0),
  CONSTRAINT frame_delta_x_non_negative CHECK (delta_x >= 0)
);

INSERT INTO frame
  (project_id, ship_id, index, delta_x)
VALUES
  (NULL, 1, 0, 0),
  (NULL, 1, 1, 5.87);

SELECT * FROM frame WHERE ship_id=1;

SELECT (id, index, delta_x) FROM frame WHERE ship_id=1;
