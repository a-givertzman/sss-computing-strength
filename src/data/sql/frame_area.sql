DROP TABLE IF EXISTS frame_area;

CREATE TABLE if not exists frame_area (
  id INT GENERATED ALWAYS AS IDENTITY,
  frame_id INT NOT NULL,
  key REAL NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT frame_area_pk PRIMARY KEY (id),
  CONSTRAINT frame_area_key_unique UNIQUE (frame_id, key),
  CONSTRAINT frame_area_key_non_negative CHECK (key >= 0),
  CONSTRAINT frame_area_value_non_negative CHECK (value >= 0)
);

INSERT INTO frame_area
  (frame_id, key, value)
VALUES
  (1, 0, 0),
  (1, 10, 40);

SELECT * FROM frame_area WHERE frame_id=1;

SELECT (key, value) FROM frame_area WHERE frame_id=1;
