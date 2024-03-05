DROP TABLE IF EXISTS frame_area;

CREATE TABLE if not exists frame_area (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  frame_index INT NOT NULL,
  key REAL NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT frame_area_pk PRIMARY KEY (id),
  CONSTRAINT frame_area_key_unique UNIQUE (frame_index, key),
  CONSTRAINT frame_area_key_non_negative CHECK (key >= 0),
  CONSTRAINT frame_area_value_non_negative CHECK (value >= 0)
);

INSERT INTO frame_area
  (project_id, ship_id, frame_index, key, value)
VALUES
  (NULL, 1, 0, 0, 0),
  (NULL, 1, 0, 10, 40),
  (NULL, 1, 1, 0, 0),
  (NULL, 1, 1, 10, 40);

SELECT * FROM frame_area WHERE ship_id=1;

SELECT frame_index, key, value FROM frame_area WHERE ship_id=1;
