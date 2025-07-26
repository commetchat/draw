pub fn timestamp_to_z_offset(timestamp: f64) -> f32 {
    const SECONDS_PER_YEAR: f64 = 31556952.0;
    const START_TIME: f64 = 1740000000.0;
    const END_TIME: f64 = START_TIME + (SECONDS_PER_YEAR * 20.0);

    let z_offset = inverse_lerp(START_TIME, END_TIME, timestamp);

    return z_offset as f32;
}

fn inverse_lerp(a: f64, b: f64, v: f64) -> f64 {
    (v - a) / (b - a)
}
