const epsilon: f64 = 0.0001;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < epsilon
}
