DROP TABLE IF EXISTS ship;

CREATE TABLE if not exists ship (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  key TEXT NOT NULL,
  value TEXT NOT NULL,
  value_type TEXT NOT NULL,
  name TEXT NOT NULL,
  unit TEXT,
  CONSTRAINT ship_pk PRIMARY KEY (id),
  CONSTRAINT ship_unique UNIQUE (ship_id, key),
  CONSTRAINT ship_key_check CHECK(char_length(key) <= 50),
  CONSTRAINT ship_value_check CHECK(char_length(value) <= 50),
  CONSTRAINT ship_type_check CHECK(char_length(value_type) <= 10),
  CONSTRAINT ship_name_check CHECK(char_length(name) <= 50),
  CONSTRAINT ship_unit_check CHECK(char_length(unit) <= 10)
);

INSERT INTO ship
  (project_id, ship_id, key, value, value_type, name, unit)
VALUES
  (NULL, 1, 'name', 'M/V "YURIY ARSHENEVSKIY"', 'text', 'Name of ship', NULL),
  (NULL, 1, 'navigation_area', 'R2', 'text', 'Type of navigation area', NULL),
  (NULL, 1, 'water_density', '1.025', 'real', 'Water Density', 'g/ml'),
  (NULL, 1, 'n_parts', '20', 'int', 'Number of Parts', NULL),
  (NULL, 1, 'const_mass_shift_x', '1.05', 'real', 'Center of mass shift x', 'm'),
  (NULL, 1, 'const_mass_shift_y', '0', 'real', 'Center of mass shift y', 'm'),
  (NULL, 1, 'const_mass_shift_z', '5.32', 'real', 'Center of mass shift z', 'm'),
  (NULL, 1, 'windage', '1089.79', 'real', 'Total windage area', 'm^2'),
  (NULL, 1, 'windage_shift_x', '3.79', 'real', 'Center of mass shift x', 'm'),
  (NULL, 1, 'windage_shift_z', '6.51', 'real', 'Center of mass shift y', 'm'),
  (NULL, 1, 'windage_icing', '1193.58', 'real', 'Total windage area with icing', 'm^2'),
  (NULL, 1, 'windage_icing_shift_x', '3.46', 'real', 'Center of mass shift x', 'm'),
  (NULL, 1, 'windage_icing_shift_z', '7.03', 'real', 'Center of mass shift y', 'm');

SELECT * FROM ship WHERE ship_id=1;

TRUNCATE TABLE ship;
