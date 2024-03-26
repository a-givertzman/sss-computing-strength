-- Безразмерный множитель Х_2 в зависимости от 
-- коэфициента общей полноты судна C_b
-- Табл. 2.1.5.1-2
TRUNCATE TABLE multipler_x2;

INSERT INTO multipler_x2
  (c_b, x2)
VALUES
  (0.45, 0.75),
  (0.5, 0.82),
  (0.55, 0.89),
  (0.6, 0.95),
  (0.65, 0.97),
  (0.7, 1.0);