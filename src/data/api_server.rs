//! Функции для работы с АПИ-сервером
use api_tools::client::{api_query::*, api_request::ApiRequest};
use crate::{data::{api_server::{loads::{CenterVolumeArray, CenterVolumeData, FreeMomentInertiaArray, FreeMomentInertiaData, LoadConstantArray, LoadSpaceArray, TankDataArray}, serde_parser::IFromJson}, structs::*}, error::{self, Error}};
use std::{thread, time};

pub struct ApiServer {
    database: String,
    request: Option<ApiRequest>,
}
///
impl ApiServer {
    pub fn new(database: String) -> Self {
        Self {
            database,
            request: None,
        }
    }
    ///
    pub fn fetch(&mut self, sql: &str) -> Result<Vec<u8>, Error> {
        if let Some(request) = self.request.as_mut() {
            let result = request.fetch(
                &ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), sql)), false),
                true,
            )?;
            let millis = time::Duration::from_millis(100);
            thread::sleep(millis);
      //      dbg!(sql, &String::from_utf8(result.clone()));
            let json: serde_json::Value = serde_json::from_slice(&result)?;
            let error_mess = json
                .get("error")
                .ok_or(Error::FromString(format!("ApiServer can't get error:{}", json)))?
                .get("message")
                .ok_or(Error::FromString(format!("ApiServer can't get error message:{}", json)))?
                .as_str()
                .ok_or(Error::FromString(format!("ApiServer can't get error message str:{}", json)))?;
            if error_mess.len() > 0 {
                return Err( Error::FromString( error_mess.to_owned() ));
            }
            Ok(result)
        } else {
            self.request = Some(ApiRequest::new(
                "parent",
                "0.0.0.0:8080",
                "auth_token",
                ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), "")), false),
                true,
                false,
            ));
            self.fetch(sql)
        }
    }
}

/*
/// Вспомогательная функция для выполнения запроса к апи-серверу
fn api_server.fetch(
    request: &mut ApiRequest,
    database: impl Into<String>,
    sql: impl Into<String>,
) -> Result<Vec<u8>, Error> {
    let query = ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, sql)), false);
    Ok(request.fetch(&query, true)?)
}
*/
/*
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
    
            include_str!("../../src/data/sql/tables/ship.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/center_waterline.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/rad_long.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/mean_draught.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/center_draught.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/frame.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/frame_area.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/load_space.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/load_constant.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/tank.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/tank_center.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);

    let query = ApiQuery::new(
        ApiQueryKind::Sql(ApiQuerySql::new(
    
            include_str!("../../src/data/sql/tables/tank_inertia.sql"),
        )),
        false,
    );
    dbg!(&String::from_utf8(request.fetch(&query, false)?)?);
    Ok(())
}
*/

/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub fn get_data(db_name: &str, ship_id: usize) -> Result<ParsedShipData, Error> {
    log::info!("input_api_server read begin");    
    let mut api_server = ApiServer::new(db_name.to_owned()); 
    let ship_data = ShipArray::parse(&api_server.fetch(
        &format!("SELECT key, value, value_type FROM ship_parameters WHERE ship_id={};", ship_id)
    )?)?;
    let load_space = LoadSpaceArray::parse(&api_server.fetch(
        &format!("SELECT space_id, key, value, value_type FROM load_space WHERE ship_id={};", ship_id)
    )?)?;
    let navigation_area_param = NavigationAreaArray::parse(&api_server.fetch(
        &format!("SELECT area, p_v, m FROM navigation_area;")
    )?)?;
    let multipler_x1 = MultiplerX1Array::parse(&api_server.fetch(
        &format!("SELECT key, value FROM multipler_x1;")
    )?)?;
    let multipler_x2 = MultiplerX2Array::parse(&api_server.fetch(
        &format!("SELECT key, value FROM multipler_x2;")
    )?)?;
    let multipler_s = MultiplerSArray::parse(&api_server.fetch(
        &format!("SELECT area, t, s FROM multipler_s;")
    )?)?;
    let coefficient_k = CoefficientKArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM coefficient_k;")
    )?)?;
    let coefficient_k_theta = CoefficientKThetaArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM coefficient_k_theta;")
    )?)?;
    let bounds = ComputedFrameDataArray::parse(&api_server.fetch(
        &format!("SELECT index, key, value FROM computed_frame WHERE ship_id={};", ship_id)
    )?)?;
    let center_waterline = CenterWaterlineArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM center_waterline WHERE ship_id={};", ship_id)
    )?)?;
    let waterline_length = WaterlineLengthArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM waterline_length WHERE ship_id={};", ship_id)
    )?)?;
    let waterline_breadth = WaterlineBreadthArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM waterline_breadth WHERE ship_id={};", ship_id)
    )?)?;
    let volume_shift = VolumeShiftArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM volume_shift WHERE ship_id={};", ship_id)
    )?)?;
    let center_draught_shift = CenterDraughtShiftDataArray::parse(&api_server.fetch(
        &format!("SELECT key, value_x, value_y, value_z FROM center_draught WHERE ship_id={};", ship_id)
    )?)?;
    let mean_draught = MeanDraughtDataArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM mean_draught WHERE ship_id={};", ship_id)
    )?)?;
    let rad_long = RadLongDataArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM rad_long WHERE ship_id={};", ship_id)
    )?)?;
    let rad_cross = RadLongDataArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM rad_cross WHERE ship_id={};", ship_id)
    )?)?;
    let pantocaren = PantocarenDataArray::parse(&api_server.fetch(
        &format!("SELECT draught, roll, moment FROM pantocaren WHERE ship_id={};", ship_id)
    )?)?;
    let flooding_angle = FloodingAngleDataArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM flooding_angle WHERE ship_id={};", ship_id)
    )?)?;
    let entry_angle = EntryAngleDataArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM entry_angle WHERE ship_id={};", ship_id)
    )?)?;
    let delta_windage_area = DeltaWindageAreaDataArray::parse(&api_server.fetch(
        &format!("SELECT key, value FROM delta_windage_area WHERE ship_id={};", ship_id)
    )?)?;
    let delta_windage_moment = DeltaWindageMomentDataArray::parse(&api_server.fetch(
        &format!("SELECT draught, value_x, value_z FROM delta_windage_moment WHERE ship_id={};", ship_id)
    )?)?;
    let physical_frame = FrameDataArray::parse(&api_server.fetch(
        &format!("SELECT index, key, value FROM physical_frame WHERE ship_id={};", ship_id)
    )?)?;
    let theoretical_frame = FrameDataArray::parse(&api_server.fetch(
        &format!("SELECT index, key, value FROM theoretical_frame WHERE ship_id={};", ship_id)
    )?)?;
    let frame_area = FrameAreaData::new(
        FrameAreaArray::parse(&api_server.fetch(
        &format!("SELECT frame_index, key, value FROM frame_area WHERE ship_id={};", ship_id)
        )?)?
        .data(),
    );
    let load_constant = LoadConstantArray::parse(&api_server.fetch(
        &format!("SELECT frame_space_index, key, value FROM load_constant WHERE ship_id={};", ship_id)
    )?)?;
    let tank = TankDataArray::parse(&api_server.fetch(
        &format!("SELECT tank_id, key, value FROM tank WHERE ship_id={};", ship_id)
    )?)?;
    let tank_center = CenterVolumeData::new(
        CenterVolumeArray::parse(&api_server.fetch(       
            &format!("SELECT tank_id, key, value_x, value_y, value_z FROM tank_center WHERE tank_id={};", ship_id)
        )?)?
        .data(),
    );
    let tank_inertia = FreeMomentInertiaData::new(
        FreeMomentInertiaArray::parse(&api_server.fetch(  
            &format!("SELECT tank_id, key, value_x, value_y FROM tank_center WHERE tank_id={};", ship_id)
        )?)?
        .data(),
    );
    let area_h_str = HStrAreaDataArray::parse(&api_server.fetch(
        &format!("SELECT name, value, bound_x1, bound_x2, bound_type FROM horizontal_area_strength WHERE ship_id={};", ship_id)
    )?)?;
    //  dbg!(&area_h_str);
    log::info!("input_api_server area_h_str read ok");
    let area_h_stab = HStabAreaDataArray::parse(&api_server.fetch(
        &format!("SELECT name, value, shift_x, shift_y, shift_z FROM horizontal_area_stability WHERE ship_id={};", ship_id)
    )?)?;
    //  dbg!(&area_h_stab);
    log::info!("input_api_server area_h_stab read ok");
    let area_v = VerticalAreaDataArray::parse(&api_server.fetch(
        &format!("SELECT name, value, shift_z, bound_x1, bound_x2, bound_type FROM vertical_area WHERE ship_id={};", ship_id)
    )?)?;
    //  dbg!(&area_v);
    log::info!("input_api_server vertical_area read ok");
    log::info!("input_api_server read ok");
    ParsedShipData::parse(
        navigation_area_param,
        multipler_x1,
        multipler_x2,
        multipler_s,
        coefficient_k,
        coefficient_k_theta,
        ship_id,
        ship_data,
        bounds,
        center_waterline,
        waterline_length,
        waterline_breadth,
        volume_shift,
        rad_long,
        rad_cross,
        mean_draught,
        center_draught_shift,
        pantocaren,
        flooding_angle,
        entry_angle,
        delta_windage_area,
        delta_windage_moment,
        physical_frame,
        theoretical_frame,
        frame_area,
        load_constant,
        load_space,
        tank,
        tank_center,
        tank_inertia,
        area_h_stab,
        area_h_str,
        area_v,
    )
}

/// Запись данных расчета прочности в БД
pub fn send_strenght_data(db_name: &str, ship_id: usize, shear_force: &Vec<f64>, bending_moment: &Vec<f64>) -> Result<(), error::Error> {
    let tmp: Vec<_> = shear_force.clone().into_iter().zip(bending_moment.into_iter()).collect();
    let mut string = 
        "INSERT INTO strength_result (ship_id, index, value_shear_force, value_bending_moment) VALUES".to_owned() + 
        &tmp.iter().enumerate().map(|(i, (v1, v2))| format!("({ship_id}, {i}, {v1}, {v2}),\n" ) ).collect::<String>();
    string.pop();
    string.pop();
    string.push(';');

 //   dbg!(&string);
    ApiServer::new(db_name.to_owned()).fetch(&string,)?;

    Ok(())
}


/// Запись данных расчета остойчивости в БД
pub fn send_stability_data(db_name: &str, data: Vec<String>) -> Vec<Result<Vec<u8>, error::Error>> {
    log::info!("input_api_server read begin");   
    let result = data.iter().map(|string| {
        ApiServer::new(db_name.to_owned()).fetch(string,)
    }).collect::<Vec<_>>();
    result
}
/*
/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub async fn async_get_data(db_name: &str, ship_id: usize) -> Result<ParsedShipData, Error> {
    log::info!("input_api_server read begin");
    let navigation_area = async_query(

        &format!("SELECT area, p_v, m FROM navigation_area;"),
    );
    log::info!("input_api_server navigation_area read ok");
    let ship = async_query(

        &format!("SELECT key, value FROM ship WHERE ship_id={};", ship_id),
    );
    log::info!("input_api_server ship read ok");
    let center_waterline = async_query(

        &format!(
            "SELECT key, value FROM center_waterline WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server center_waterline read ok");
    let center_draught_shift = async_query(

        &format!(
            "SELECT key, value_x, value_y, value_z FROM center_draught WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server center_draught_shift read ok");
    let mean_draught = async_query(

        &format!(
            "SELECT key, value FROM mean_draught WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server mean_draught read ok");
    let rad_long = async_query(

        &format!("SELECT key, value FROM rad_long WHERE ship_id={};", ship_id),
    );
    log::info!("input_api_server rad_long read ok");
    let rad_cross = async_query(

        &format!("SELECT key, value FROM rad_cross WHERE ship_id={};", ship_id),
    );
    log::info!("input_api_server rad_cross read ok");
    let pantocaren = async_query(

        &format!(
            "SELECT draught, roll, moment FROM pantocaren WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server pantocaren read ok");
    let frame = async_query(

        &format!(
            "SELECT index, key, value FROM frame WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server frame read ok");
    let frame_area = async_query(

        &format!(
            "SELECT frame_index, key, value FROM frame_area WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server frame_area read ok");
    let load_space = async_query(

        &format!(
            "SELECT space_id, key, value FROM load_space WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server load_space read ok");
    let load_constant = async_query(

        &format!(
            "SELECT frame_space_index, key, value FROM load_constant WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server load_constant read ok");
    let tank = async_query(

        &format!(
            "SELECT tank_id, key, value FROM tank WHERE ship_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server tank read ok");
    let tank_center = async_query(

        &format!(
            "SELECT tank_id, key, value_x, value_y, value_z FROM tank_center WHERE tank_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server tank_center read ok");
    let tank_inertia = async_query(

        &format!(
            "SELECT tank_id, key, value_x, value_y FROM tank_center WHERE tank_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server tank_inertia read ok");
    let (
        navigation_area,
        ship,
        center_waterline,
        center_draught_shift,
        mean_draught,
        rad_long,
        rad_cross,
        pantocaren,
        frame,
        frame_area,
        load_space,
        load_constant,
        tank,
        tank_center,
        tank_inertia,
    ) = futures::join!(
        navigation_area,
        ship,
        center_waterline,
        center_draught_shift,
        mean_draught,
        rad_long,
        rad_cross,
        pantocaren,
        frame,
        frame_area,
        load_space,
        load_constant,
        tank,
        tank_center,
        tank_inertia,
    );
    log::info!("input_api_server read ok");
    ParsedShipData::parse(
        NavigationAreaArray::parse(&navigation_area?)?,
        ship_id,
        ShipArray::parse(&ship?)?,
        CenterWaterlineArray::parse(&center_waterline?)?,
        RadLongDataArray::parse(&rad_long?)?,
        RadCrossDataArray::parse(&rad_cross?)?,
        MeanDraughtDataArray::parse(&mean_draught?)?,
        CenterDraughtShiftDataArray::parse(&center_draught_shift?)?,
        PantocarenDataArray::parse(&pantocaren?)?,
        FrameDataArray::parse(&frame?)?,
        FrameAreaData::new(
            FrameAreaArray::parse(&frame_area?)?
            .data()),
            LoadConstantArray::parse(&load_constant?)?,
        LoadSpaceArray::parse(&load_space?)?,
        TankDataArray::parse(&tank?)?,
        CenterVolumeData::new(
            CenterVolumeArray::parse(&tank_center?)?
            .data()),
            FreeMomentInertiaData::new(
                FreeMomentInertiaArray::parse(&tank_inertia?)?.data()),
    )
}

/// Вспомогательная функция для выполнения запроса к апи-серверу
async fn async_query(database: impl Into<String>, sql: String) -> Result<Vec<u8>, Error> {
    let query = ApiQuery::new(ApiQueryKind::Sql(ApiQuerySql::new(database, sql.clone())), false);
    let mut request = ApiRequest::new(
        "parent",
        "0.0.0.0:8080",
        "auth_token",
        query.clone(),
        false,
        false,
    );
    Ok(request.fetch(&query, false)?)
}
*/
