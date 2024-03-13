-- Общие данные по кораблю

INSERT INTO ship
  (project_id, ship_id, key, value, name, unit)
VALUES
  (NULL, 1, 'water_density', 1.025, 'Water Density', 'g/ml'),
  (NULL, 1, 'n_parts', 20, 'Number of Parts', NULL),
  (NULL, 1, 'const_mass_shift_x', 1.05, 'Center of mass shift x', NULL),
  (NULL, 1, 'const_mass_shift_y', 0, 'Center of mass shift y', NULL),
  (NULL, 1, 'const_mass_shift_z', 5.32, 'Center of mass shift z', NULL);
