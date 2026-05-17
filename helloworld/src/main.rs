use equationsolver::{solve_linear_equation, solve_quadratic_equation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let choices = vec![1, 2];
    println!("Hiện tại chỉ mới xong được bậc nhất và bậc hai, còn bậc ba với bậc bốn là còn cực kì lâu nữa.");
    let choice = inquire::Select::new("Chọn một trong hai mode: 1 hoặc 2:\nMode 1 là giải phương trình ax + b = 0 (a != 0), mode 2 là giải phương trình ax² + bx + c = 0 (a != 0).", choices).prompt()?;
    if choice == 1 {
        println!("Giải phương trình bậc nhất ax + b = 0 (a != 0)");
        let a = inquire::Text::new("Nhập số a: ").prompt()?.parse::<f64>()?;
        let b = inquire::Text::new("Nhập số b: ").prompt()?.parse::<f64>()?;
        match solve_linear_equation(a, b) {
            None => println!("Phương trình vô nghiệm. (hoặc vô số nghiệm, tôi không biết :D)"),
            Some(x) => println!("Phương trình có nghiệm x = {}", x)
        }
    }
    else {
        println!("Giải phương trình bậc hai ax² + bx + c = 0 (a != 0) (còn nhiều bug chưa sửa, mong thông cảm, có gì report các bug đó lên GitHub giùm tôi)");
        let a = inquire::Text::new("Nhập số a: ").prompt()?.parse::<f64>()?;
        let b = inquire::Text::new("Nhập số b: ").prompt()?.parse::<f64>()?;
        let c = inquire::Text::new("Nhập số c: ").prompt()?.parse::<f64>()?;
        match solve_quadratic_equation(a, b, c) {
            None => println!("Phương trình vô nghiệm."),
            Some((x1, x2)) => {
                if x1 == x2 {
                    println!("Phương trình có nghiệm kép x₁ = x₂ = {}", x2)
                }
                else {
                    println!("Phương trình có hai nghiệm phân biệt:\nx₁ = {}\nx₂ = {}", x1, x2)
                }
            }
        }
    }
    Ok(())
}
