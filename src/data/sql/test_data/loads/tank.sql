-- Переменный жидкий груз - цистерна

INSERT INTO tank
  (project_id, ship_id, tank_id, key, value)
VALUES
  (NULL, 1, 1, 'density', 1),
  (NULL, 1, 1, 'volume', 1000),
  (NULL, 1, 1, 'bound_x1', -10),
  (NULL, 1, 1, 'bound_x2', 4);
  