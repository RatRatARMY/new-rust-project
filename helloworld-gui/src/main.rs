pub mod events;
pub mod logic;
pub mod page;

fn main() -> iced::Result {
    iced::application(|| logic::AppState::default(), logic::update, logic::view)
        .title("Giải các loại phương trình từ bậc nhất đến bậc bốn.")
        .run()
}
