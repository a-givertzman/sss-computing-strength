DROP TABLE IF EXISTS frame;

CREATE TABLE if not exists frame (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  frame_id INT NOT NULL,
  key TEXT NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT frame_pk PRIMARY KEY (id),
  CONSTRAINT frame_index_unique UNIQUE (frame_id, key),
  CONSTRAINT frame_key_check CHECK(char_length(key) <= 50)
);

INSERT INTO frame
  (project_id, ship_id, index, delta_x)
VALUES
  (NULL, 1, 0, 0),
  (NULL, 1, 1, 5.87);

SELECT * FROM frame WHERE frame_id=1;

SELECT (frame_id, key, delta_x) FROM frame WHERE ship_id=1;
