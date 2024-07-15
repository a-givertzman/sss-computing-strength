//! Функции для работы с АПИ-сервером
use crate::{
    data::{
        api_server::{loads::LoadConstantArray, serde_parser::IFromJson},
        structs::*,
    },
    error::{self, Error},
    CriterionData,
};
use api_tools::client::{api_query::*, api_request::ApiRequest};
use loads::{CompartmentArray, LoadCargoArray};
use std::{thread, time};

pub struct ApiServer {
    database: String,
    host: String,
    port: String,
    request: Option<ApiRequest>,
}
///
impl ApiServer {
    pub fn new(database: String, host: String, port: String) -> Self {
        Self {
            database,
            host,
            port,
            request: None,
        }
    }
    ///
    pub fn fetch(&mut self, sql: &str) -> Result<Vec<u8>, Error> {
        if let Some(request) = self.request.as_mut() {
            let result = request.fetch(
                &ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), sql)),
                    false,
                ),
                true,
            )?;
            let millis = time::Duration::from_millis(100);
            thread::sleep(millis);
            //      dbg!(sql, &String::from_utf8(result.clone()));
            let json: serde_json::Value = serde_json::from_slice(&result)?;
            let error_mess = json
                .get("error")
                .ok_or(Error::FromString(format!(
                    "ApiServer can't get error:{}",
                    json
                )))?
                .get("message")
                .ok_or(Error::FromString(format!(
                    "ApiServer can't get error message:{}",
                    json
                )))?
                .as_str()
                .ok_or(Error::FromString(format!(
                    "ApiServer can't get error message str:{}",
                    json
                )))?;
            if error_mess.len() > 0 {
                return Err(Error::FromString(error_mess.to_owned()));
            }
            Ok(result)
        } else {
            self.request = Some(ApiRequest::new(
                "parent",
                self.host.clone() + ":" + &self.port,
                "auth_token",
                ApiQuery::new(
                    ApiQueryKind::Sql(ApiQuerySql::new(self.database.clone(), "")),
                    false,
                ),
                true,
                false,
            ));
            self.fetch(sql)
        }
    }
}

/// Чтение данных из БД. Функция читает данные за несколько запросов,
/// парсит их и проверяет данные на корректность.
pub fn get_data(
    api_server: &mut ApiServer,
    ship_id: usize,
) -> Result<ParsedShipData, Error> {
    log::info!("input_api_server read begin");
    let ship_parameters = ShipArray::parse(&api_server.fetch(&format!(
        "SELECT key, value, value_type FROM ship_parameters WHERE ship_id={};",
        ship_id
    ))?)?;
    let navigation_area_param = NavigationAreaArray::parse(
        &api_server.fetch(&format!("SELECT area, p_v, m FROM navigation_area;"))?,
    )?;
    let multipler_x1 = MultiplerX1Array::parse(
        &api_server.fetch(&format!("SELECT key, value FROM multipler_x1;"))?,
    )?;
    let multipler_x2 = MultiplerX2Array::parse(
        &api_server.fetch(&format!("SELECT key, value FROM multipler_x2;"))?,
    )?;
    let multipler_s = MultiplerSArray::parse(
        &api_server.fetch(&format!("SELECT area, t, s FROM multipler_s;"))?,
    )?;
    let coefficient_k = CoefficientKArray::parse(
        &api_server.fetch(&format!("SELECT key, value FROM coefficient_k;"))?,
    )?;
    let coefficient_k_theta = CoefficientKThetaArray::parse(
        &api_server.fetch(&format!("SELECT key, value FROM coefficient_k_theta;"))?,
    )?;
    let icing = IcingArray::parse(&api_server.fetch(&format!("SELECT key, value FROM icing;"))?)?;
    let bounds = ComputedFrameDataArray::parse(&api_server.fetch(&format!(
        "SELECT index, start_x, end_x FROM computed_frame_space WHERE ship_id={};",
        ship_id
    ))?)?;
    let center_waterline = CenterWaterlineArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM center_waterline WHERE ship_id={};",
        ship_id
    ))?)?;
    let waterline_length = WaterlineLengthArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM waterline_length WHERE ship_id={};",
        ship_id
    ))?)?;
    let waterline_breadth = WaterlineBreadthArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM waterline_breadth WHERE ship_id={};",
        ship_id
    ))?)?;
    let waterline_area = WaterlineAreaArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM waterline_area WHERE ship_id={};",
        ship_id
    ))?)?;
    let volume_shift = VolumeShiftArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM volume_shift WHERE ship_id={};",
        ship_id
    ))?)?;
    let center_draught_shift = CenterDraughtShiftDataArray::parse(&api_server.fetch(&format!(
        "SELECT key, value_x, value_y, value_z FROM center_draught WHERE ship_id={};",
        ship_id
    ))?)?;
    let mean_draught = MeanDraughtDataArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM mean_draught WHERE ship_id={};",
        ship_id
    ))?)?;
    let rad_long = RadLongDataArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM rad_long WHERE ship_id={};",
        ship_id
    ))?)?;
    let rad_trans = RadTransDataArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM rad_trans WHERE ship_id={};",
        ship_id
    ))?)?;
    let h_subdivision = MetacentricHeightSubdivisionArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM min_metacentric_height_subdivision WHERE ship_id={ship_id};"
    ))?)?;
    //    dbg!(&h_subdivision);
    log::info!("input_api_server h_subdivision read ok");
    let pantocaren = PantocarenDataArray::parse(&api_server.fetch(&format!(
        "SELECT draught, roll, moment FROM pantocaren WHERE ship_id={};",
        ship_id
    ))?)?;
    let flooding_angle = FloodingAngleDataArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM flooding_angle WHERE ship_id={};",
        ship_id
    ))?)?;
    let entry_angle = EntryAngleDataArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM entry_angle WHERE ship_id={};",
        ship_id
    ))?)?;
    let delta_windage_area = DeltaWindageAreaDataArray::parse(&api_server.fetch(&format!(
        "SELECT key, value FROM delta_windage_area WHERE ship_id={};",
        ship_id
    ))?)?;
    let delta_windage_moment = DeltaWindageMomentDataArray::parse(&api_server.fetch(&format!(
        "SELECT draught, value_x, value_z FROM delta_windage_moment WHERE ship_id={};",
        ship_id
    ))?)?;
    let bonjean_frame = FrameIndexDataArray::parse(&api_server.fetch(&format!(
        "SELECT frame_index, pos_x FROM bonjean_frame WHERE ship_id={ship_id};"
    ))?)?;
    let frame_area = FrameAreaDataArray::parse(&api_server.fetch(&format!(
        "SELECT frame_index, draft, area FROM frame_area WHERE ship_id={};",
        ship_id
    ))?)?;
    let draft_mark = DraftMarkDataArray::parse(&api_server.fetch(&format!(
        "SELECT name, x, y, z FROM draft_mark WHERE ship_id={};",
        ship_id
    ))?)?;
    let cargo = LoadCargoArray::parse(&api_server.fetch(&format!(
        "SELECT c.name AS name, \
                c.mass AS mass, \
                c.bound_x1 AS bound_x1, \
                c.bound_x2 AS bound_x2, \
                c.bound_y1 AS bound_y1, \
                c.bound_y2 AS bound_y2, \
                c.bound_z1 AS bound_z1, \
                c.bound_z2 AS bound_z2, \
                c.mass_shift_x AS mass_shift_x, \
                c.mass_shift_y AS mass_shift_y, \
                c.mass_shift_z AS mass_shift_z, \
                c.horizontal_area AS horizontal_area, \
                c.horizontal_area_shift_x AS horizontal_area_shift_x, \
                c.horizontal_area_shift_y AS horizontal_area_shift_y, \
                c.vertical_area AS vertical_area, \
                c.vertical_area_shift_x AS vertical_area_shift_x, \
                c.vertical_area_shift_y AS vertical_area_shift_y, \
                c.vertical_area_shift_z AS vertical_area_shift_z, \
                c.loading_type::TEXT \
            FROM cargo WHERE ship_id={ship_id};"
        ),
    )?)?;
    let compartment = CompartmentArray::parse(&api_server.fetch(&format!(
        "SELECT space_id AS name, \
                name AS name, \
                mass AS name, \
                density AS name, \
                volume AS name, \
                bound_x1 AS name, \
                bound_x2 AS name, \
                mass_shift_x AS name, \
                mass_shift_y AS name, \
                mass_shift_z AS name, \
                m_f_s_y AS name, \
                m_f_s_x AS name, \
                grain_moment AS name, \
                loading_type::TEXT AS name, \
                physical_type::TEXT \
            FROM compartment WHERE ship_id={ship_id} AND active=TRUE AND mass>0;"
    ))?)?;
    let load_constant = LoadConstantArray::parse(&api_server.fetch(&format!(
        "SELECT mass, bound_x1, bound_x2, loading_type::TEXT FROM load_constant WHERE ship_id={};",
        ship_id
    ))?)?;
    let area_h_str = HStrAreaArray::parse(&api_server.fetch(
        &format!("SELECT name, value, bound_x1, bound_x2 FROM horizontal_area_strength WHERE ship_id={};", ship_id)
    )?)?;
    log::info!("input_api_server area_h_str read ok");
    let area_h_stab = HStabAreaArray::parse(&api_server.fetch(
        &format!("SELECT name, value, shift_x, shift_y, shift_z FROM horizontal_area_stability WHERE ship_id={};", ship_id)
    )?)?;
    log::info!("input_api_server area_h_stab read ok");
    let area_v_str = strength::VerticalAreaArray::parse(&api_server.fetch(
        &format!("SELECT name, value, shift_z, bound_x1, bound_x2 FROM vertical_area_strength WHERE ship_id={};", ship_id)
    )?)?;
    log::info!("input_api_server area_v_str read ok");
    let area_v_stab = stability::VerticalAreaArray::parse(&api_server.fetch(
        &format!("SELECT draught, area, moment_x, moment_z FROM vertical_area_stability WHERE ship_id={};", ship_id)
    )?)?;
    log::info!("input_api_server area_v_stab read ok");
    log::info!("input_api_server read ok");
    ParsedShipData::parse(
        navigation_area_param,
        multipler_x1,
        multipler_x2,
        multipler_s,
        coefficient_k,
        coefficient_k_theta,
        icing,
        ship_id,
        ship_parameters,
        bounds,
        center_waterline,
        waterline_length,
        waterline_breadth,
        waterline_area,
        volume_shift,
        rad_long,
        rad_trans,
        h_subdivision,
        mean_draught,
        center_draught_shift,
        pantocaren,
        flooding_angle,
        entry_angle,
        delta_windage_area,
        delta_windage_moment,
        bonjean_frame,
        frame_area,
        draft_mark,
        cargo,
        compartment,
        load_constant,
        area_h_stab,
        area_h_str,
        area_v_stab,
        area_v_str,
    )
}

/// Запись данных расчета прочности в БД
pub fn send_strenght_data(
    api_server: &mut ApiServer,
    ship_id: usize,
    results: Vec<(String, Vec<f64>)>,
) -> Result<(), error::Error> {
    log::info!("send_strenght_data begin");
    let mut full_sql = "DO $$ BEGIN ".to_owned();
    full_sql += &format!("DELETE FROM result_strength WHERE ship_id={ship_id};");
    full_sql += &format!(" INSERT INTO result_strength (ship_id, index");
    full_sql += &results.iter().map(|(k, _)| format!(", {k}") ).collect::<String>();
    full_sql += ") VALUES";

    for i in 0..results[0].1.len() {
        full_sql += &format!(" ({ship_id}, {i}," );
        full_sql += &results.iter().map(|(_, v)| format!(" {},", v[i])).collect::<String>();
        full_sql.pop();
        full_sql += &format!(")," );        
    }
    full_sql.pop();
    full_sql.push(';');

/*     results.iter().for_each(|(k, v)| {
        full_sql += &format!(" INSERT INTO result_strength (ship_id, index, {}) VALUES", k);
        v.iter().enumerate().for_each(|(i, v)| {
            full_sql += &format!(" ({ship_id}, {i}, {v}),");
        });
        full_sql.pop();
        full_sql.push(';');
    }); */

    full_sql += " END$$;";
    //   dbg!(&string);
    api_server.fetch(&full_sql)?;
    log::info!("send_strenght_data end");
    Ok(())
}
/// Запись данных расчета остойчивости в БД
pub fn send_stability_data(
    api_server: &mut ApiServer,
    ship_id: usize,
    data: Vec<CriterionData>,
) -> Result<(), error::Error> {
    log::info!("send_stability_data begin");
    let mut full_sql = "DO $$ BEGIN ".to_owned();
    full_sql += &format!("DELETE FROM result_stability WHERE ship_id={ship_id};");
    data.into_iter().for_each(|v| {
        full_sql += " INSERT INTO result_stability "; 
            if let Some(error) = v.error_message {
                full_sql += &format!("(ship_id, criterion_id, result, target, error_message) VALUES ({ship_id}, {}, {}, {}, '{}');", v.criterion_id, v.result, v.target, error);
            } else {
                full_sql += &format!("(ship_id, criterion_id, result, target) VALUES ({ship_id}, {}, {}, {});", v.criterion_id, v.result, v.target);
            }
    });
    full_sql += " END$$;";
    api_server.fetch(&full_sql)?;
    log::info!("send_stability_data end");
    Ok(())
}
/// Запись данных расчета плечей остойчивости в БД
pub fn send_stability_diagram(
    api_server: &mut ApiServer,
    ship_id: usize,
    data: Vec<(f64, f64, f64)>,
) -> Result<(), error::Error> {
    log::info!("send_stability_diagram begin");
    let mut full_sql = "DO $$ BEGIN ".to_owned();
    full_sql += &format!("DELETE FROM stability_diagram WHERE ship_id={ship_id};");
    full_sql += " INSERT INTO stability_diagram (ship_id, angle, value_dso, value_ddo) VALUES"; 
    data.into_iter().for_each(|(angle, value_dso, value_ddo) | {
        full_sql += &format!(" ({ship_id}, {angle}, {value_dso}, {value_ddo}),");
    });
    full_sql.pop();
    full_sql.push(';');
    full_sql += " END$$;";
    api_server.fetch(&full_sql)?;
    log::info!("send_stability_diagram end");
    Ok(())
}
/// Запись данных расчета остойчивости в БД
pub fn send_parameters_data(
    api_server: &mut ApiServer,
    ship_id: usize,
    data: Vec<(usize, f64)>,
) -> Result<(), error::Error> {
    log::info!("send_parameters_data begin");
    let mut full_sql = "DO $$ BEGIN ".to_owned();
    full_sql += &format!("DELETE FROM parameter_data WHERE ship_id={ship_id};");
    if data.len() > 0 {
        full_sql += " INSERT INTO parameter_data (ship_id, parameter_id, result) VALUES";
        data.into_iter().for_each(|v| {
            full_sql += &format!(" ({ship_id}, {}, {}),", v.0, v.1);
        });
        full_sql.pop();
        full_sql.push(';');
    }
    full_sql += " END$$;";
    api_server.fetch(&full_sql)?;
    log::info!("send_parameters_data end");
    Ok(())
}
/// Запись данных расчета расчет уровня заглубления 
/// для координат отметок заглубления на корпусе судна
pub fn send_draft_mark(
    api_server: &mut ApiServer,
    ship_id: usize,
    data: Vec<(String, (f64, f64, f64))>,
) -> Result<(), error::Error> {
    log::info!("send_draft_mark begin");
    let mut full_sql = "DO $$ BEGIN ".to_owned();
    full_sql += &format!("DELETE FROM draft_mark_result WHERE ship_id={ship_id};");
    if data.len() > 0 {
        full_sql += " INSERT INTO draft_mark_result (ship_id, name, x, y, draft_value) VALUES";
        data.into_iter().for_each(|(name, (x, y, draft_value))| {
            full_sql += &format!(" ({ship_id}, '{name}', {x}, {y}, {draft_value}),");
        });
        full_sql.pop();
        full_sql.push(';');
    }
    full_sql += " END$$;";
    api_server.fetch(&full_sql)?;
    log::info!("send_parameters_data end");
    Ok(())
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
            "SELECT key, x, y, z FROM center_draught WHERE ship_id={};",
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
    let rad_trans = async_query(

        &format!("SELECT key, value FROM rad_trans WHERE ship_id={};", ship_id),
    );
    log::info!("input_api_server rad_trans read ok");
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
            "SELECT tank_id, key, x, y, z FROM tank_center WHERE tank_id={};",
            ship_id
        ),
    );
    log::info!("input_api_server tank_center read ok");
    let tank_inertia = async_query(

        &format!(
            "SELECT tank_id, key, x, y FROM tank_center WHERE tank_id={};",
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
        rad_trans,
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
        rad_trans,
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
        RadCrossDataArray::parse(&rad_trans?)?,
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
