-- Переменный жидкий груз - цистерна
TRUNCATE TABLE tank;

INSERT INTO tank
  (project_id, ship_id, tank_id, key, value)
VALUES
  (NULL, 1, 1, 'density', 1),
  (NULL, 1, 1, 'volume', 0),
  (NULL, 1, 1, 'bound_x1', -10),
  (NULL, 1, 1, 'bound_x2', 4);
  