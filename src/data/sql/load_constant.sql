-- Постоянная нагрузка на судно, распределенная по шпациям
DROP TABLE IF EXISTS load_constant;

CREATE TABLE if not exists load_constant (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  frame_space_index INT NOT NULL,
  key TEXT NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT load_constant_pk PRIMARY KEY (id),
  CONSTRAINT load_constant_key_unique UNIQUE (ship_id, frame_space_index, key),
  CONSTRAINT load_constant_key_check CHECK(char_length(key) <= 50)
);

INSERT INTO load_constant
  (project_id, ship_id, frame_space_index, key, value)
VALUES
  (NULL, 1, 0, 'hull', 90.95),
  (NULL, 1, 0, 'equipment', 48.39),
  (NULL, 1, 1, 'hull', 114.81),
  (NULL, 1, 1, 'equipment', 129.48);

SELECT * FROM load_constant WHERE ship_id=1;