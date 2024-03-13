//! Функции для работы с АПИ-сервером
use api_tools::client::{api_query::*, api_request::ApiRequest};

use crate::{data::structs::*, error::Error};

/// Создание тестовой БД
#[allow(dead_code)]
pub fn create_test_db(db_name: &str) -> Result<(), Error> {
    //   let script = include_str!("../../src/data/sql/tables/create_postgres_db.sql");

    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(db_name, "")), false),
        false,
        false,
    );

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/ship.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/center_waterline.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/rad_long.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/mean_draught.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/center_draught.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/frame.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/frame_area.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/load_space.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/load_constant.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/tank.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/tank_center.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
            db_name,
            include_str!("../../src/data/sql/tables/tank_inertia.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);
    Ok(())
}

/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub fn get_data(db_name: &str, ship_id: usize) -> Result<ParsedShipData, Error> {
    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(db_name, "")), false),
        false,
        false,
    );
    log::info!("input_api_server read begin");
    let ship = ShipArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!("SELECT key, value FROM ship WHERE ship_id={};", ship_id),
    )?)?;
    //dbg!(&ship);
    log::info!("input_api_server ship read ok");
    let center_waterline = CenterWaterlineArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT key, value FROM center_waterline WHERE ship_id={};",
            ship_id
        ),
    )?)?;
//    dbg!(&center_waterline);
    log::info!("input_api_server center_waterline read ok");
    let center_draught_shift = CenterDraughtShiftDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT key, value_x, value_y, value_z FROM center_draught WHERE ship_id={};",
            ship_id
        ),
    )?)?;
  //  dbg!(&center_draught_shift);
    log::info!("input_api_server center_draught_shift read ok");
    let mean_draught =
        MeanDraughtDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT key, value FROM mean_draught WHERE ship_id={};",
            ship_id
        ),
    )?)?;
//    dbg!(&mean_draught);
    log::info!("input_api_server mean_draught read ok");
    let rad_long = RadLongDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!("SELECT key, value FROM rad_long WHERE ship_id={};", ship_id),
    )?)?;
//    dbg!(&rad_long);
    log::info!("input_api_server rad_long read ok");
    let frame = FrameDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT index, key, value FROM frame WHERE ship_id={};",
            ship_id
        ),
    )?)?;
//    dbg!(&frame);
    log::info!("input_api_server frame read ok");
    let frame_area = FrameAreaData::new(FrameAreaArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT frame_index, key, value FROM frame_area WHERE ship_id={};",
            ship_id
        ),
    )?)?.data());
//    dbg!(&frame_area);
    log::info!("input_api_server frame_area read ok");
    let load_space = LoadSpaceArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT space_id, key, value FROM load_space WHERE ship_id={};",
            ship_id
        ),
    )?)?;
//    dbg!(&load_space);
    log::info!("input_api_server load_space read ok");
    let load_constant = LoadConstantArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT frame_space_index, key, value FROM load_constant WHERE ship_id={};",
            ship_id
        ),
    )?)?;
//    dbg!(&load_constant);
    log::info!("input_api_server load_constant read ok");
    let tank = TankDataArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT tank_id, key, value FROM tank WHERE ship_id={};",
            ship_id
        ),
    )?)?;
   // dbg!(&tank);
    log::info!("input_api_server tank read ok");
    let tank_center = CenterVolumeData::new(
        CenterVolumeArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT tank_id, key, value_x, value_y, value_z FROM tank_center WHERE tank_id={};",
            ship_id
        ),
    )?)?.data());
   // dbg!(&tank_center);
    log::info!("input_api_server tank_center read ok");
    let tank_inertia = FreeMomentInertiaData::new(
        FreeMomentInertiaArray::parse(&fetch_query(
        &mut request,
        db_name,
        format!(
            "SELECT tank_id, key, value_x, value_y FROM tank_center WHERE tank_id={};",
            ship_id
        ),
    )?)?.data());
  //  dbg!(&tank_inertia);
    log::info!("input_api_server tank_inertia read ok");
    log::info!("input_api_server read ok");
    ParsedShipData::parse(
        ship_id,
        ship,
        center_waterline,
        rad_long,
        mean_draught,
        center_draught_shift,
        frame,
        frame_area,
        load_constant,
        load_space,
        tank,
        tank_center,
        tank_inertia,
    )
}
/// Вспомогательная функция для выполнения запроса к апи-серверу
fn fetch_query(
    request: &mut ApiRequest,
    database: impl Into<String>,
    sql: impl Into<String>,
) -> Result<String, Error> {
    let query = ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, sql)), false);
    Ok(String::from_utf8(request.fetch(&query, false)?)?)
}
