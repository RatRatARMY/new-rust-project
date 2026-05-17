use equationsolver::solve_linear_equation;

#[derive(Debug, Clone)]
pub enum Message {
    SolveEquation,
    AChanged(String),
    BChanged(String)
}
#[derive(Debug, Default)]
pub struct AppState {
    a: String, // thực tế là f64.
    b: String, // thực tế là f64.
    res: String
}
pub fn update(state: &mut AppState, message: Message) {
    match message {
        Message::AChanged(value) => state.a = value,
        Message::BChanged(value) => state.b = value,
        Message::SolveEquation => {
            let a = state.a.parse::<f64>().unwrap_or(0f64);
            let b = state.b.parse::<f64>().unwrap_or(0f64);
            match solve_linear_equation(a, b) {
                None => state.res = String::from("Phương trình vô nghiệm."),
                Some(x) => state.res = format!("Phương trình có nghiệm x = {:.7}", x)
            }
        }
    }
}
pub fn view(state: &AppState) -> iced::Element<Message> {
    iced::widget::column![
        iced::widget::text("Xin chào thế giới!"),
        iced::widget::text_input("Nhập a: ", &state.a).on_input(Message::AChanged),
        iced::widget::text_input("Nhập b: ", &state.b).on_input(Message::BChanged),
        iced::widget::button("Giải phương trình ax + b = 0 (a != 0)").on_press(Message::SolveEquation),
        iced::widget::text(&state.res)
    ].into()
}