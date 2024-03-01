//! Структура для ввода данных

use crate::error::Error as MyError;
use futures::FutureExt;
use log::error;

/// Общая структура для ввода данных. Содержит все данные
/// для расчетов, при заполнении проверяет данные на корректность.
#[derive(Debug)]
pub struct InputData {
    /// разбиение на шпации - количество
    pub n_parts: u32,
    /// плотность воды
    pub water_density: f64,
    /// длинна корпуса судна
    pub ship_length: f64,
    /// кривая отстояния центра тяжести ватерлинии по длине от миделя  
    pub center_waterline: Vec<(f64, f64)>,
    /// кривая продольного метацентрического радиуса
    pub rad_long: Vec<(f64, f64)>,
    /// кривая средней осадки
    pub mean_draught: Vec<(f64, f64)>,
    /// кривая отстояния центра величины погруженной части судна
    pub center_shift: Vec<(f64, f64, f64, f64)>,
}

pub async fn get_data() -> Result<InputData, MyError> {
    let (client, connection) = tokio_postgres::Config::new()
        .user("postgres")
        .password("123qwe")
        .host("localhost")
        .port(5432)
        .dbname("test")
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
    };
    for row in ship_parameters {
        match row.get("key") {
            "n_parts" => {
                let value: f64 = row.get("value");
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
    dbg!(&data);
    Ok(data)
}
