use crate::data::parse_input::ParsedFramesData;

pub(crate) fn frames() -> ParsedFramesData {
    let data = r#"
    {
        "frames": [ 
            {
                "index": 0, 
                "immersion_area": [[0.0, 0.0], [1.0, 1.0], [5.0, 10.0]]
            }
        ]
    }"#;        

    ParsedFramesData::parse(&data).expect("parse error")
}
