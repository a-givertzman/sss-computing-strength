-- Безразмерный множитель S в зависимости от 
-- от района плавания судна и периода качки Т
-- Табл. 2.1.5.1-3
TRUNCATE TABLE multipler_s;

INSERT INTO multipler_s
  (area, t, s)
VALUES
  ('Unlimited', 5, 0.1),
  ('Unlimited', 6, 0.1),
  ('Unlimited', 7, 0.098),
  ('Unlimited', 8, 0.093),
  ('Unlimited', 10, 0.079),
  ('Unlimited', 12, 0.065),
  ('Unlimited', 14, 0.053),
  ('Unlimited', 16, 0.044),
  ('Unlimited', 18, 0.038),
  ('Unlimited', 20, 0.035),
  ('Other', 5, 0.1),
  ('Other', 6, 0.093),
  ('Other', 7, 0.083),
  ('Other', 8, 0.073),
  ('Other', 10, 0.053),
  ('Other', 12, 0.04),
  ('Other', 14, 0.035),
  ('Other', 16, 0.035),
  ('Other', 18, 0.035),
  ('Other', 20, 0.035);