/*  I did not write this code!
 *
 *  I got it from this stack overflow post
 *
 *  https://stackoverflow.com/questions/27928/calculate-distance-between-two-latitude-longitude-points-haversine-formula
 */
fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let p: f64 = 0.017453292519943295;    // Math.PI / 180
    let a: f64 = 0.5 - f64::cos((lat2 - lat1) * p) / 2.0 + f64::cos(lat1 * p) *
                 f64::cos(lat2 * p) * (1.0 - f64::cos((lon2 - lon1) * p)) / 2.0;

    return 12742.0 * f64::asin(f64::sqrt(a)); // 2 * R; R = 6371 km
}

fn km_to_mi(km: f64) -> f64 {
    return km * 0.6213712;
}

pub fn haversine_mi(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    return km_to_mi(haversine(lat1, lon1, lat2, lon2)).abs();
}
