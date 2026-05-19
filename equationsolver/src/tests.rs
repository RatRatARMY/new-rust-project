#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_linear_equation() {
        use crate::solve_linear_equation;
        let result1 = solve_linear_equation(87f64, 29f64).unwrap();
        let result2 = solve_linear_equation(32f64, -76f64).unwrap();
        assert_eq!(result1, -29f64 / 87f64);
        assert_eq!(result2, 76f64 / 32f64);
    }
    #[test]
    fn test_solve_quadratic_equation() {
        use crate::solve_quadratic_equation;
        let res = solve_quadratic_equation(3f64, 7f64, -1f64).unwrap();
        let x1 = res.0;
        let x2 = res.1;
        assert_eq!(x1, (-7f64 + 61f64.sqrt()) / 6f64);
        assert_eq!(x2, (-7f64 - 61f64.sqrt()) / 6f64);
    }
}
