pub fn distance((p1x, p1y): (f64, f64), (p2x, p2y): (f64, f64)) -> f64 {
    let dx = p1x - p2x;
    let dy = p1y - p2y;
    (dx * dx + dy * dy).sqrt()
}