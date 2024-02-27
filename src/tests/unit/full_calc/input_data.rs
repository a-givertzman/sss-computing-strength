use crate::data::parse_input::ParsedInputData;

pub(crate) fn input_data() -> ParsedInputData {
    let data = r#"
    {
        "project_name": "YURIY ARSHENEVSKIY",
        "ship_name": "YURIY ARSHENEVSKIY",
        "n_parts": 20,
        "water_density": 1.025
    }"#;        

    ParsedInputData::parse(&data).expect("parse error")
}