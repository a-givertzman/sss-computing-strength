-- Постоянная нагрузка на судно, распределенная по шпациям

INSERT INTO load_space
  (project_id, ship_id, space_id, key, value)
VALUES
  (NULL, 1, 1, 'mass', 1000),
  (NULL, 1, 1, 'bound_x1', -10),
  (NULL, 1, 1, 'bound_x2', 4),
  (NULL, 1, 1, 'bound_y1', -2),
  (NULL, 1, 1, 'bound_y2', 0),
  (NULL, 1, 1, 'center_x', -2),
  (NULL, 1, 1, 'center_y', -1),
  (NULL, 1, 1, 'center_z', 1);