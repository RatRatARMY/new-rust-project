pub mod logic;
fn main() -> iced::Result {
    println!("Phiên bản GUI này chắc có thể không hoàn thành được nếu như chỉ trong hè này, mong thông cảm nha các anh em dev của tôi.");
    iced::application(|| logic::AppState::default(), logic::update, logic::view)
        .title("Giải các loại phương trình từ bậc nhất đến bậc bốn.")
        .run()
}
