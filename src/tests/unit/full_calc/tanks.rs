use crate::data::parse_input::ParsedTanksData;

pub(crate) fn tanks() -> ParsedTanksData {
    let data = r#"
    {
        "tanks": [
            {
                "density": 0.7,
                "volume": 10.0,
                "bound": [10.0, 20.0, 5.0, 10.0], 
                "center": [[0.0, 15.0, 7.5, -1.0], [10.0, 15.0, 7.5, 2.0]],
                "free_surf_inertia": [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0]]       
            }
        ]
    }"#;        

    ParsedTanksData::parse(&data).expect("parse error")
}