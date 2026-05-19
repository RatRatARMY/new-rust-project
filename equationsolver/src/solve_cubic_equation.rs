use crate::solve_quadratic_equation;
/// Giải phương trình bậc ba `ax³ + bx² + cx + d = 0 (a != 0)`<br>
/// Chúng ta sẽ chỉ quan tâm đến nghiệm thực và bỏ qua nghiệm phức. (đơn giản thôi: số phức quá phức tạp để xử lí)
pub fn solve_cubic_equation(a: f64, b: f64, c: f64, d: f64) -> Option<(f64, f64, f64)> {
    if a == 0f64 {
        return Some((
            0f64,
            solve_quadratic_equation(b, c, d)?.0,
            solve_quadratic_equation(b, c, d)?.1,
        ));
    }
    let b = b / a;
    let c = c / a;
    let d = d / a;
    let p = c - ((b * b) / 3f64);
    let q = ((2f64 * b.powf(3f64)) / 27f64) - ((b * c) / 3f64) + d;
    let delta = (q / 2.0).powf(2f64) + (p / 3.0).powf(3f64);
    let u: f64;
    if delta > 0f64 {
        u = (-q / 2f64 + delta.sqrt()).cbrt();
        let v = (-q / 2f64 - delta.sqrt()).cbrt();
        let y = u + v;
        let x = y - b / 3f64;
        Some((x, x, x))
    } else if delta == 0f64 {
        u = (q / 2f64).cbrt();
        let y1 = 2f64 * u;
        let y2 = -u;
        Some((y1 - b / 3f64, y2 - b / 3f64, y2 - b / 3f64))
    } else {
        let r = (-p / 3f64).sqrt();
        let phi = (-(q / 2f64) / r.powf(3f64)).acos();
        let x1 = 2f64 * r * ((phi + 2f64 * std::f64::consts::PI) / 3f64).cos() - b / 3f64;
        let x2 = 2f64 * r * ((phi + 4f64 * std::f64::consts::PI) / 3f64).cos() - b / 3f64;
        let x3 = 2f64 * r * (phi / 3f64).cos() - b / 3f64;
        Some((x1, x2, x3))
    }
}
