-- Постоянная нагрузка на судно, распределенная по шпациям
DROP TABLE IF EXISTS load_stock;

CREATE TABLE if not exists load_stock (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  frame_space_index INT NOT NULL,
  key REAL NOT NULL,
  value REAL NOT NULL,
  CONSTRAINT load_stock_pk PRIMARY KEY (id),
  CONSTRAINT load_stock_key_unique UNIQUE (ship_id, frame_space_index, key),
  CONSTRAINT load_stock_key_check_low CHECK( key >= 0 ),
  CONSTRAINT load_stock_key_check_hight CHECK( key <= 100 )
);

INSERT INTO load_stock
  (project_id, ship_id, frame_space_index, key, value)
VALUES
  (NULL, 1, 0, 0, 0),
  (NULL, 1, 0, 100, 117.86),
  (NULL, 1, 1, 0, 0),
  (NULL, 1, 1, 100, 41.77);

SELECT * FROM load_stock WHERE ship_id=1;