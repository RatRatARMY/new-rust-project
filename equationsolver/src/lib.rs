//! Giải các loại phương trình từ bậc nhất đến bậc bốn.<br>
//! Lưu ý: Trong các output thì chỉ có nghiệm thực, còn nghiệm phức sẽ không có.
pub mod solve_cubic_equation;
pub mod solve_linear_equation;
pub mod solve_quadratic_equation;


pub use solve_cubic_equation::solve_cubic_equation;
pub use solve_linear_equation::solve_linear_equation;
pub use solve_quadratic_equation::solve_quadratic_equation;
