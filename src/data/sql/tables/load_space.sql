DROP TABLE IF EXISTS load_space;

CREATE TABLE if not exists load_space (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  space_id INT NOT NULL,
  key TEXT NOT NULL,
  value TEXT NOT NULL,
  value_type TEXT NOT NULL,
  unit TEXT,
  CONSTRAINT load_space_pk PRIMARY KEY (id),
  CONSTRAINT load_space_key_unique UNIQUE (ship_id, space_id, key),
  CONSTRAINT ship_value_check CHECK(char_length(value) <= 50),
  CONSTRAINT ship_type_check CHECK(char_length(value_type) <= 10),
  CONSTRAINT load_space_key_check CHECK(char_length(key) <= 50),
  CONSTRAINT ship_unit_check CHECK(char_length(unit) <= 10)
);

INSERT INTO load_space
  (project_id, ship_id, space_id, key, value, value_type, unit)
VALUES
  (NULL, 1, 24, 'name', 'Цистерна расходного топлива 1 ЛБ', 'text', NULL),
  (NULL, 1, 24, 'mass', '5.51', 'real', 'ton'),
  (NULL, 1, 24, 'bound_x1', '-55.4', 'real', 'm'),
  (NULL, 1, 24, 'bound_x2', '-53', 'real', 'm'),
  (NULL, 1, 24, 'center_x', '-54.39', 'real', 'm'),
  (NULL, 1, 24, 'center_y', '-1.73', 'real', 'm'),
  (NULL, 1, 24, 'center_z', '5.30', 'real', 'm'),
  (NULL, 1, 24, 'm_f_s_y', '0.34', 'real', 'm'),  
  (NULL, 1, 24, 'm_f_s_x', '0.4', 'real', 'm');

SELECT * FROM load_space WHERE ship_id=1;

TRUNCATE TABLE load_space;
