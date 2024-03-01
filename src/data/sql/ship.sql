DROP TABLE IF EXISTS ship CASCADE;

CREATE TABLE if not exists ship (
  ship_id INT NOT NULL UNIQUE,
  CONSTRAINT ship_pk PRIMARY KEY (ship_id)
);

INSERT INTO ship
  (ship_id)
VALUES
  (1);

SELECT * FROM ship WHERE ship_id=1;

SELECT (ship_id) FROM ship WHERE ship_id=1;