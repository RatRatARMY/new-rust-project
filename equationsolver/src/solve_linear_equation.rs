pub fn solve_linear_equation(a: f64, b: f64) -> Option<f64> {
    if a == 0f64 {
        return None;
    }
    Some(-b / a)
}
