-- Общие данные по кораблю
TRUNCATE TABLE ship;

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
