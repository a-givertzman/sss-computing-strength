-- Давление ветра p_v и добавка на порывистость m 
-- в зависимости от района плавания судна,
-- Табл. 2.1.4.1
TRUNCATE TABLE navigation_area;

INSERT INTO navigation_area
  (area, p_v, m)
VALUES
  ('Unlimited', 504, 0.5),
  ('R1', 353, 0.5),
  ('R2', 252, 0.52),
  ('R2-RSN', 252, 0.52),
  ('R2-RSN(4,5)', 166, 0.54),
  ('R3-RSN', 119, 0.55);