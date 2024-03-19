DROP TABLE IF EXISTS pantocaren;

CREATE TABLE if not exists pantocaren (
  id INT GENERATED ALWAYS AS IDENTITY,
  project_id INT,
  ship_id INT NOT NULL,
  draught FLOAT8 NOT NULL, 
  roll FLOAT8 NOT NULL,
  moment FLOAT8 NOT NULL,
  CONSTRAINT pantocaren_pk PRIMARY KEY (id)
);

INSERT INTO pantocaren
  (project_id, ship_id, draught, roll, moment)
VALUES
  (NULL, 1, 1, 0.1, 10),
  (NULL, 1, 2, 0.2, 20);

SELECT * FROM pantocaren WHERE ship_id=1;

TRUNCATE TABLE pantocaren;