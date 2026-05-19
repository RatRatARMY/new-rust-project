use equationsolver::solve_cubic_equation::solve_cubic_equation;
use equationsolver::{solve_linear_equation, solve_quadratic_equation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let choices = vec![1, 2, 3];
    println!("Hiện tại chỉ mới xong thực sự được bậc nhất, bậc hai và bậc ba; còn bậc bốn thì còn rất lâu nữa.");
    let choice = inquire::Select::new("Chọn một trong ba mode: 1, 2 hoặc 3:", choices).prompt()?;
    if choice == 1 {
        println!("Giải phương trình bậc nhất ax + b = 0 (a != 0)");
        let a = inquire::Text::new("Nhập số a: ").prompt()?.parse::<f64>()?;
        let b = inquire::Text::new("Nhập số b: ").prompt()?.parse::<f64>()?;
        match solve_linear_equation(a, b) {
            None => println!("Phương trình vô nghiệm. (hoặc vô số nghiệm, tôi không biết :D)"),
            Some(x) => println!("Phương trình có nghiệm x = {}", x),
        }
    } else if choice == 2 {
        println!("Giải phương trình bậc hai ax² + bx + c = 0 (a != 0) (còn nhiều bug chưa sửa, mong thông cảm, có gì report các bug đó lên GitHub giùm tôi)");
        let a = inquire::Text::new("Nhập số a: ").prompt()?.parse::<f64>()?;
        let b = inquire::Text::new("Nhập số b: ").prompt()?.parse::<f64>()?;
        let c = inquire::Text::new("Nhập số c: ").prompt()?.parse::<f64>()?;
        match solve_quadratic_equation(a, b, c) {
            None => println!("Phương trình vô nghiệm."),
            Some((x1, x2)) => {
                if x1 == x2 {
                    println!("Phương trình có nghiệm kép x₁ = x₂ = {}", x2)
                } else {
                    println!(
                        "Phương trình có hai nghiệm phân biệt:\nx₁ = {}\nx₂ = {}",
                        x1, x2
                    )
                }
            }
        }
    } else {
        println!("Giải phương trình bậc ba ax³ + bx² + cx + d = 0 (a != 0) (cực kì nhiều bug chưa sửa, mong thông cảm, có gì report các bug đó lên GitHub giùm tôi)");
        let a = inquire::Text::new("Nhập số a: ").prompt()?.parse::<f64>()?;
        let b = inquire::Text::new("Nhập số b: ").prompt()?.parse::<f64>()?;
        let c = inquire::Text::new("Nhập số c: ").prompt()?.parse::<f64>()?;
        let d = inquire::Text::new("Nhập số d: ").prompt()?.parse::<f64>()?;
        match solve_cubic_equation(a, b, c, d) {
            None => {} // không thể tới được vì phương trình bậc ba luôn có nghiệm.
            Some((x1, x2, x3)) => {
                println!("Phương trình bậc ba này có nghiệm (gì tôi không biết :D):\nx₁ = {}\nx₂ = {}\nx₃ = {}", x1, x2, x3)
            }
        }
    }
    Ok(())
}
