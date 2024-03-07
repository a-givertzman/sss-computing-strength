-- Удаление всех данных из таблицы по шпангоутам
TRUNCATE TABLE frame;

-- Удаление всех данных из таблицы погруженной площади по шпангоутам
TRUNCATE TABLE frame_area;

-- Удаление данных по шпангоуту из таблицы погруженной площади по шпангоутам
DELETE FROM frame_area WHERE EXISTS
    (SELECT * FROM frame_area
             WHERE frame_index=20);
