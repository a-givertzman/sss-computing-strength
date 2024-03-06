//! Функции для работы с АПИ-сервером
use api_tools::client::{api_query::*, api_request::ApiRequest};

use crate::{data::structs::*, error::Error};

/// Создание тестовой БД
#[allow(dead_code)]
pub fn create_test_db(db_name: &str) -> Result<(), Error> {
    //   let script = include_str!("../../src/data/sql/create_postgres_db.sql");

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/ship.sql"),
        )),
        false,
    );

    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        query.clone(),
        false,
        false,
    );

    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/center_waterline.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/rad_long.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/mean_draught.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/center_shift.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/frame.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/frame_area.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/load_space.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tank.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tank_center.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tank_inertia.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);
    Ok(())
}
/// Чтение данных из БД. Функция читает данные за несколько запросов, 
/// парсит их и проверяет данные на корректность.
pub fn get_data(db_name: &str, ship_id: usize) -> Result<ParsedShipData, Error> {
    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!("SELECT key, value FROM ship WHERE ship_id={};", ship_id),
        )),
        false,
    );

    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        query.clone(),
        false,
        false,
    );
    let ship = ShipArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT key, value FROM center_waterline WHERE ship_id={};",
                ship_id
            ),
        )),
        false,
    );
    let center_waterline =
        CenterWaterlineArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT key, value_x, value_y, value_z FROM center_shift WHERE ship_id={};",
                ship_id
            ),
        )),
        false,
    );
    let center_shift =
        CenterShiftDataArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT key, value FROM mean_draught WHERE ship_id={};",
                ship_id
            ),
        )),
        false,
    );
    let mean_draught =
        MeanDraughtDataArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!("SELECT key, value FROM rad_long WHERE ship_id={};", ship_id),
        )),
        false,
    );
    let rad_long = RadLongDataArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT index, key, value FROM frame WHERE ship_id={};",
                ship_id
            ),
        )),
        false,
    );
    let frame = FrameDataArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT frame_index, key, value FROM frame_area WHERE ship_id={};",
                ship_id
            ),
        )),
        false,
    );
    let frame_area = FrameAreaData::new(
        FrameAreaArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?.data(),
    );

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT space_id, key, value FROM load_space WHERE ship_id={};",
                ship_id
            ),
        )),
        false,
    );
    let load_space = LoadSpaceArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT tank_id, key, value FROM tank WHERE ship_id={};",
                ship_id
            ),
        )),
        false,
    );
    let tank = TankDataArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?;

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT tank_id, key, value_x, value_y, value_z FROM tank_center WHERE tank_id={};",
                ship_id
            ),
        )),
        false,
    );
    let tank_center = CenterVolumeData::new(
        CenterVolumeArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?.data(),
    );

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            format!(
                "SELECT tank_id, key, value_x, value_y FROM tank_center WHERE tank_id={};",
                ship_id
            ),
        )),
        false,
    );
    let tank_inertia = FreeMomentInertiaData::new(
        FreeMomentInertiaArray::parse(&String::from_utf8(request.fetch(&query, false)?)?)?.data(),
    );
    ParsedShipData::parse(
        ship_id,
        ship,
        center_waterline,
        rad_long,
        mean_draught,
        center_shift,
        frame,
        frame_area,
        load_space,
        tank,
        tank_center,
        tank_inertia,
    )
}
