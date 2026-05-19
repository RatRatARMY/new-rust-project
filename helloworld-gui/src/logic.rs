use equationsolver::{solve_cubic_equation, solve_linear_equation, solve_quadratic_equation};
#[derive(Debug)]
pub struct AppState {
    current_page: crate::page::Pages,
    linear_equation_state: LinearEquationState,
    quadratic_equation_state: QuadraticEquationState,
    cubic_equation_state: CubicEquationState,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            current_page: crate::page::Pages::Home,
            linear_equation_state: LinearEquationState::default(),
            quadratic_equation_state: QuadraticEquationState::default(),
            cubic_equation_state: CubicEquationState::default(),
        }
    }
}
#[derive(Debug, Default)]
pub struct LinearEquationState {
    a: String,
    b: String,
    res: String,
}
#[derive(Debug, Default)]
pub struct QuadraticEquationState {
    a: String,
    b: String,
    c: String,
    res: String,
}
#[derive(Debug, Default)]
pub struct CubicEquationState {
    a: String,
    b: String,
    c: String,
    d: String,
    res: String,
}
pub fn linear_equation_update(state: &mut LinearEquationState, message: crate::events::Events) {
    match message {
        crate::events::Events::AChanged(value) => state.a = value,
        crate::events::Events::BChanged(value) => state.b = value,
        crate::events::Events::SolveEquation => {
            let a = state.a.parse::<f64>().unwrap_or(0f64);
            let b = state.b.parse::<f64>().unwrap_or(0f64);
            match solve_linear_equation(a, b) {
                None => state.res = String::from("Phương trình vô nghiệm."),
                Some(x) => state.res = format!("Phương trình có nghiệm x = {}", x),
            }
        }
        _ => {}
    }
}
pub fn linear_equation_view(
    state: &LinearEquationState,
) -> iced::Element<'_, crate::events::Events> {
    iced::widget::column![
        iced::widget::text("Xin chào thế giới!"),
        iced::widget::button("Quay lại")
            .on_press(crate::events::Events::ChangePage(crate::page::Pages::Home)),
        iced::widget::text_input("Nhập a: ", &state.a).on_input(crate::events::Events::AChanged),
        iced::widget::text_input("Nhập b: ", &state.b).on_input(crate::events::Events::BChanged),
        iced::widget::button("Giải phương trình ax + b = 0 (a != 0)")
            .on_press(crate::events::Events::SolveEquation),
        iced::widget::text(&state.res)
    ]
    .into()
}
pub fn quadratic_equation_update(
    state: &mut QuadraticEquationState,
    message: crate::events::Events,
) {
    match message {
        crate::events::Events::AChanged(value) => state.a = value,
        crate::events::Events::BChanged(value) => state.b = value,
        crate::events::Events::CChanged(value) => state.c = value,
        crate::events::Events::SolveEquation => {
            let a = state.a.parse::<f64>().unwrap_or(0f64);
            let b = state.b.parse::<f64>().unwrap_or(0f64);
            let c = state.c.parse::<f64>().unwrap_or(0f64);
            match solve_quadratic_equation(a, b, c) {
                None => state.res = String::from("Phương trình vô nghiệm."),
                Some((x1, x2)) => {
                    if x1 == x2 {
                        state.res = format!("Phương trình có nghiệm kép:\nx₁ = x₂ = {}", x2)
                    } else {
                        state.res = format!(
                            "Phương trình có hai nghiệm phân biệt:\nx₁ = {}\nx₂ = {}",
                            x1, x2
                        )
                    }
                }
            }
        }
        _ => {}
    }
}
pub fn quadratic_equation_view(
    state: &QuadraticEquationState,
) -> iced::Element<'_, crate::events::Events> {
    iced::widget::column![
        iced::widget::text("Giải phương trình bậc hai ax² + bx + c = 0 (a != 0)"),
        iced::widget::button("Quay lại")
            .on_press(crate::events::Events::ChangePage(crate::page::Pages::Home)),
        iced::widget::text_input("Nhập a: ", &state.a).on_input(crate::events::Events::AChanged),
        iced::widget::text_input("Nhập b: ", &state.b).on_input(crate::events::Events::BChanged),
        iced::widget::text_input("Nhập c: ", &state.c).on_input(crate::events::Events::CChanged),
        iced::widget::button("Giải phương trình ax² + bx + c = 0 (a != 0)")
            .on_press(crate::events::Events::SolveEquation),
        iced::widget::text(&state.res)
    ]
    .into()
}
pub fn cubic_equation_update(state: &mut CubicEquationState, message: crate::events::Events) {
    match message {
        crate::events::Events::AChanged(value) => state.a = value,
        crate::events::Events::BChanged(value) => state.b = value,
        crate::events::Events::CChanged(value) => state.c = value,
        crate::events::Events::DChanged(value) => state.d = value,
        crate::events::Events::SolveEquation => {
            let a = state.a.parse::<f64>().unwrap_or(0f64);
            let b = state.b.parse::<f64>().unwrap_or(0f64);
            let c = state.c.parse::<f64>().unwrap_or(0f64);
            let d = state.d.parse::<f64>().unwrap_or(0f64);
            match solve_cubic_equation(a, b, c, d) {
                None => {}
                Some((x1, x2, x3)) => state.res = format!("Phương trình có các nghiệm (nghiệm loại gì tôi không biết, nhưng đại loại là có nghiệm):\nx₁ = {}\nx₂ = {}\nx₃ = {}", x1, x2, x3)
            }
        }
        _ => {}
    }
}
pub fn cubic_equation_view(state: &CubicEquationState) -> iced::Element<'_, crate::events::Events> {
    iced::widget::column![
        iced::widget::text("Giải phương trình bậc ba ax³ + bx² + cx + d = 0 (a != 0)"),
        iced::widget::button("Quay lại.")
            .on_press(crate::events::Events::ChangePage(crate::page::Pages::Home)),
        iced::widget::text_input("Nhập số a: ", &state.a).on_input(crate::events::Events::AChanged),
        iced::widget::text_input("Nhập số b: ", &state.b).on_input(crate::events::Events::BChanged),
        iced::widget::text_input("Nhập số c: ", &state.c).on_input(crate::events::Events::CChanged),
        iced::widget::text_input("Nhập số d: ", &state.d).on_input(crate::events::Events::DChanged),
        iced::widget::button("Giải phương trình ax³ + bx² + cx + d = 0 (a != 0)")
            .on_press(crate::events::Events::SolveEquation),
        iced::widget::text(&state.res)
    ]
    .into()
}
pub fn update(state: &mut AppState, message: crate::events::Events) {
    match message {
        crate::events::Events::ChangePage(page) => state.current_page = page,
        _ => match state.current_page {
            crate::page::Pages::LinearEquation => {
                linear_equation_update(&mut state.linear_equation_state, message)
            }
            crate::page::Pages::QuadraticEquation => {
                quadratic_equation_update(&mut state.quadratic_equation_state, message)
            }
            crate::page::Pages::CubicEquation => {
                cubic_equation_update(&mut state.cubic_equation_state, message)
            }
            _ => {}
        },
    }
}
pub fn view(state: &AppState) -> iced::Element<'_, crate::events::Events> {
    match state.current_page {
        crate::page::Pages::Home => iced::widget::column![
            iced::widget::text("Giải các loại phương trình."),
            iced::widget::button("Giải phương trình bậc nhất ax + b = 0. (a != 0)").on_press(
                crate::events::Events::ChangePage(crate::page::Pages::LinearEquation)
            ),
            iced::widget::button("Giải phương trình bậc hai ax² + bx + c = 0 (a != 0)").on_press(
                crate::events::Events::ChangePage(crate::page::Pages::QuadraticEquation)
            ),
            iced::widget::button("Giải phương trình bậc ba ax³ + bx² + cx + d = 0 (a != 0)")
                .on_press(crate::events::Events::ChangePage(
                    crate::page::Pages::CubicEquation
                ))
        ]
        .into(),
        crate::page::Pages::LinearEquation => linear_equation_view(&state.linear_equation_state),
        crate::page::Pages::QuadraticEquation => {
            quadratic_equation_view(&state.quadratic_equation_state)
        }
        crate::page::Pages::CubicEquation => cubic_equation_view(&state.cubic_equation_state),
    }
}
