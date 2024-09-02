#[allow(dead_code)]
fn data() -> Vec<(f64, f64, f64)> {
    vec![
        (0.5, -173.106, -93.6799),
        (1., -66.8216, -53.0677),
        (1.5, 13.45388, 16.2197),
        (2., 57.9012, 115.3278),
        (2.5, 60.4136, 245.4224),
        (3., 16.236, 407.376),
        (3.5, -49.044, 600.728),
        (4., -107.878, 824.3113),
        (4.5, -161.684, 1078.012),
        (5., -210.654, 1361.637),
        (5.5, -255.454, 1675.663),
        (6., -297.371, 2020.581),
        (6.5, -338.01, 2395.099),
        (6.8, -362.152, 2634.193),
    ]
}

#[allow(dead_code)]
pub(crate) fn delta_windage_moment_x() -> Vec<(f64, f64)> {
    data().iter().map(|&(draught, value_x, _)| (draught, value_x) ).collect()
}

#[allow(dead_code)]
pub(crate) fn delta_windage_moment_z() -> Vec<(f64, f64)> {
    data().iter().map(|&(draught, _, value_z)| (draught, value_z) ).collect()
}
