DROP TABLE IF EXISTS navigation_area;

CREATE TABLE if not exists navigation_area (
  id INT GENERATED ALWAYS AS IDENTITY,
  area TEXT NOT NULL,
  p_v FLOAT8 NOT NULL,
  m FLOAT8 NOT NULL,
  CONSTRAINT navigation_area_pk PRIMARY KEY (id),
  CONSTRAINT navigation_area_unique UNIQUE (area),
  CONSTRAINT navigation_area_area_check CHECK(char_length(area) <= 50),
  CONSTRAINT navigation_area_p_v_non_negative CHECK (p_v > 0),
  CONSTRAINT navigation_area_m_non_negative CHECK (m >= 0)
);

INSERT INTO navigation_area
  (area, p_v, m)
VALUES
  ('Unlimited', 504, 0.5),
  ('R1', 353, 0.5),
  ('R2', 252, 0.52),
  ('R2-RSN', 252, 0.52),
  ('R2-RSN(4,5)', 166, 0.54),
  ('R3-RSN', 119, 0.55);

SELECT * FROM navigation_area;

TRUNCATE TABLE navigation_area;