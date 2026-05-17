use crate::solve_linear_equation;

pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    if a == 0f64 {
        return Some((solve_linear_equation(b, c)?, solve_linear_equation(b, c)?))
    }
    if b == 0f64 {
        return if a * c < 0f64 {
            Some(((-c / a).sqrt(), (-c / a).sqrt()))
        } else {
            None
        }
    }
    if c == 0f64 {
        return Some((0f64, solve_linear_equation(a, b)?))
    }
    let delta = b * b - 4f64 * a * c;
    if delta < 0f64 {
        None
    }
    else if delta == 0f64 {
        Some((-b / (2f64 * a), -b / (2f64 * a)))
    }
    else {
        Some(((-b + delta.sqrt()) / (2f64 * a), (-b - delta.sqrt()) / (2f64 * a)))
    }
}