use crate::data::parse_input::ParsedShipData;

#[allow(dead_code)]
pub(crate) fn ship()-> ParsedShipData {
    let data = r#"
    {
        "ship_length": 200.0, 
        "center_waterline": [[0.0, 0.0], [10.0, 1.0]],
        "rad_long": [[0.0, 0.0], [10.0, 2.0]],
        "mean_draught": [[0.0, 0.0], [10.0, 3.0]],
        "center_shift": [[0.0, 2.0, 0.0, 0.0], [10.0, 2.0, 0.0, 0.0]]
    }"#;        

    ParsedShipData::parse(&data).expect("parse error")
}