//! Структура для ввода данных

use std::{collections::HashSet, fmt::Display};

use crate::error::Error as MyError;
use futures::FutureExt;
use log::error;

/// Шпангоут
#[derive(Debug)]
pub struct FrameData {
    /// Порядковый номер шпангоута от кормы
    pub index: usize,
    /// Расстояние в продольной плоскости от предыдущего шпангоута
    pub delta_x: f32,
    /// Кривая погружаемой площади от осадки
    pub immersion_area: Vec<(f32, f32)>,
}
///
impl Display for FrameData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FrameData(index:{}, delta_x:{}, immersion_area:{})",
            self.index,
            self.delta_x,
            self.immersion_area.len()
        )
    }
}
/// Груз, конструкции корпуса, контейнер или другой твердый груз
#[derive(Debug)]
pub struct LoadSpaceData {
    /// Общая масса
    pub mass: f32,
    /// Границы груза
    pub bound: (f32, f32, f32, f32),
    /// Центер масс
    pub center: (f32, f32, f32),
}
///
impl Display for LoadSpaceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(mass:{}, bound:(x1:{}, x2:{}, y1:{}, y2:{}), center:(x:{}, y:{}, z:{}))",
            self.mass,
            self.bound.0,
            self.bound.1,
            self.bound.2,
            self.bound.3,
            self.center.0,
            self.center.1,
            self.center.2
        )
    }
}
/// Цистерна
#[derive(Debug)]
pub struct TankData {
    /// плотность жидкости в цистерне
    pub density: f32,
    /// объем жидкости в цистерне
    pub volume: f32,
    /// границы цистерны, (x1, x2, y1, y2)
    pub bound: (f32, f32, f32, f32),
    /// кривая координат центра объема жидкости в цистерне в системе координат судна
    /// (volume, x, y, z)
    pub center: Vec<(f32, f32, f32, f32)>,
    /// кривая момента инерции площади свободной поверхности жидкости
    /// (volume, x - поперечный, y - продольный)
    pub free_surf_inertia: Vec<(f32, f32, f32)>,
}
impl Display for TankData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LoadSpaceData(density:{}, volume:{}, bound:(x1:{}, x2:{}, y1:{}, y2:{}), center.len:{}, free_surf_inertia.len:{}",
            self.density,
            self.volume,
            self.bound.0,
            self.bound.1,
            self.bound.2,
            self.bound.3,
            self.center.len(),
            self.free_surf_inertia.len(),
        )
    }
}
/// Общая структура для ввода данных. Содержит все данные
/// для расчетов, при заполнении проверяет данные на корректность.
#[derive(Debug)]
pub struct InputData {
    /// разбиение на шпации - количество
    pub n_parts: u32,
    /// плотность воды
    pub water_density: f32,
    /// длинна корпуса судна
    pub ship_length: f32,
    /// кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f32, f32)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f32, f32)>,
    /// кривая средней осадки
    pub mean_draught: Vec<(f32, f32)>,
    /// кривая отстояния центра величины погруженной части судна
    pub center_shift: Vec<(f32, f32, f32, f32)>,
    /// Шпангоуты судна
    pub frames: Vec<FrameData>,
    /// Нагрузка судна без жидких грузов
    pub load_space: Vec<LoadSpaceData>,
    /// Нагрузка судна, жидкие грузы
    pub tanks: Vec<TankData>,
}

pub async fn create_test_db() -> std::result::Result<Vec<tokio_postgres::SimpleQueryMessage>, tokio_postgres::Error> {
    let my_str = include_str!("../../src/data/sql/ship.sql");
 //   let my_str = include_str!("../../src/data/sql/create_postgres_db.sql");

    let (client, connection) = tokio_postgres::Config::new()
        .user("postgres")
        .password("123qwe")
        .host("localhost")
        .port(5432)
        .dbname("test")
        .connect(tokio_postgres::NoTls)
        .await.unwrap();
    let connection = connection.map(|r| {
        if let Err(e) = r {
            error!("connection error: {}", e);
        }
    });
    tokio::spawn(connection);
    tokio::join!(client.simple_query(my_str)).0
}

pub async fn get_data(db_name: &str) -> Result<InputData, MyError> {
    let (client, connection) = tokio_postgres::Config::new()
        .user("postgres")
        .password("123qwe")
        .host("localhost")
        .port(5432)
        .dbname(db_name)
        .connect(tokio_postgres::NoTls)
        .await?;
    let connection = connection.map(|r| {
        if let Err(e) = r {
            error!("connection error: {}", e);
        }
    });
    tokio::spawn(connection);
    let ship_parameters = client.query("SELECT * FROM ship_parameters WHERE ship_id=1;", &[]);
    let center_waterline = client.query("SELECT * FROM center_waterline WHERE ship_id=1;", &[]);
    let center_shift = client.query("SELECT * FROM center_shift WHERE ship_id=1;", &[]);
    let mean_draught = client.query("SELECT * FROM mean_draught WHERE ship_id=1;", &[]);
    let rad_long = client.query("SELECT * FROM rad_long WHERE ship_id=1;", &[]);
    let (ship_parameters, center_waterline, center_shift, mean_draught, rad_long) = tokio::join!(
        ship_parameters,
        center_waterline,
        center_shift,
        mean_draught,
        rad_long
    );
    let (ship_parameters, center_waterline, center_shift, mean_draught, rad_long) = (
        ship_parameters?,
        center_waterline?,
        center_shift?,
        mean_draught?,
        rad_long?,
    );
    let mut data = InputData {
        n_parts: 0,
        water_density: 0.,
        ship_length: 0.,
        center_waterline: center_waterline
            .into_iter()
            .map(|row| (row.get("key"), row.get("value")))
            .collect(),
        rad_long: rad_long
            .into_iter()
            .map(|row| (row.get("key"), row.get("value")))
            .collect(),
        mean_draught: mean_draught
            .into_iter()
            .map(|row| (row.get("key"), row.get("value")))
            .collect(),
        center_shift: center_shift
            .into_iter()
            .map(|row| {
                (
                    row.get("key"),
                    row.get("value_x"),
                    row.get("value_y"),
                    row.get("value_z"),
                )
            })
            .collect(),
        frames: todo!(),
        load_space: todo!(),
        tanks: todo!(),
    };
    for row in ship_parameters {
        match row.get("key") {
            "n_parts" => {
                let value: f32 = row.get("value");
                data.n_parts = value as u32;
            }
            "water_density" => data.water_density = row.get("value"),
            "ship_length" => data.ship_length = row.get("value"),
            s => error!("wrong ship parameter: {}", s),
        }
    }
    if data.ship_length <= 0. {
        return Err(MyError::Parameter(format!(
            "ship_length {}",
            data.ship_length
        )));
    }
    if data.n_parts == 0 {
        return Err(MyError::Parameter(format!("n_parts {}", data.n_parts)));
    }
    if data.water_density <= 0. {
        return Err(MyError::Parameter(format!(
            "water_density {}",
            data.water_density
        )));
    }
    if data.center_waterline.len() <= 1 {
        return Err(MyError::Parameter(format!(
            "data.center_waterline.len() {}",
            data.center_waterline.len()
        )));
    }
    if data.mean_draught.len() <= 1 {
        return Err(MyError::Parameter(format!(
            "data.mean_draught.len() {}",
            data.mean_draught.len()
        )));
    }
    if data.center_shift.len() <= 1 {
        return Err(MyError::Parameter(format!(
            "data.center_shift.len() {}",
            data.center_shift.len()
        )));
    }
    if data.frames.len() <= 1 {
        return Err(MyError::Parameter(format!(
            "data.frames.len() {}",
            data.frames.len()
        )));
    }
    if let Some(frame) = data.frames.iter().find(|f| f.index >= data.frames.len()) {
        return Err(MyError::Parameter(format!(
            "frame.index {} >= frames.len() {}",
            frame.index,
            data.frames.len()
        )));
    }
    let qnt_unique_index = data
        .frames
        .iter()
        .map(|f| f.index.clone())
        .collect::<HashSet<_>>()
        .len();
    if data.frames.len() != qnt_unique_index {
        return Err(MyError::Parameter(format!("frame.index not unique")));
    }
    if let Some(frame) = data.frames.iter().find(|f| f.immersion_area.len() <= 1) {
        return Err(MyError::Parameter(format!(
            "frame.immersion_area.len <= 1 {}",
            frame
        )));
    }
    if let Some(space) = data.load_space.iter().find(|s| s.mass < 0.) {
        return Err(MyError::Parameter(format!(
            "load_space.mass error: {}",
            space
        )));
    }
    if let Some(space) = data
        .load_space
        .iter()
        .find(|s| s.bound.0 >= s.bound.1 || s.bound.2 >= s.bound.3)
    {
        return Err(MyError::Parameter(format!(
            "load_space.bound error: {}",
            space,
        )));
    }
    if let Some(tank) = data.tanks.iter().find(|t| t.density <= 0.) {
        return Err(MyError::Parameter(format!("tank.density error: {}", tank)));
    }
    if let Some(tank) = data.tanks.iter().find(|t| t.volume < 0.) {
        return Err(MyError::Parameter(format!("tank.volume error: {}", tank)));
    }
    if let Some(tank) = data
        .tanks
        .iter()
        .find(|t| t.bound.0 >= t.bound.1 || t.bound.2 >= t.bound.3)
    {
        return Err(MyError::Parameter(format!("tank.bound error: {}", tank,)));
    }
    if let Some(tank) = data.tanks.iter().find(|t| t.center.len() <= 1) {
        return Err(MyError::Parameter(format!(
            "tank.center.len() <= 1 error: {}",
            tank,
        )));
    }
    if let Some(tank) = data.tanks.iter().find(|t| t.free_surf_inertia.len() <= 1) {
        return Err(MyError::Parameter(format!(
            "tank.free_surf_inertia.len() <= 1 error: {}",
            tank,
        )));
    }
    dbg!(&data);
    Ok(data)
}
