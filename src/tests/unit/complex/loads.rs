use crate::data::parse_input::ParsedLoadsData;

#[allow(dead_code)]
pub(crate) fn loads() -> ParsedLoadsData {
    let data = r#"
    {
        "load_space": [ 
            {
                "mass": 10.0,
                "bound": [-10.0, 0.0, 0.0, 5.0], 
                "center": [0.0, 0.0, 1.0]
            }
        ]
    }"#;        

    ParsedLoadsData::parse(&data).expect("parse error")
}