fn main() {
    let (width, height): (u32, u32) = (960, 720);
    let sdl3_context = sdl3::init().unwrap();
    let video_subsystem = sdl3_context.video().unwrap();
    let window = video_subsystem.window("Catch the apple", width, height)
        .position_centered().build().unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    use sdl3::image::LoadSurface;
    let main_surface = sdl3::surface::Surface::from_file("assets/bg_image.png").unwrap();
    let cat_sprite = sdl3::surface::Surface::from_file("assets/spr_cat.png").unwrap();
    let bg_img = texture_creator.create_texture_from_surface(&main_surface).unwrap();
    let cat_img = texture_creator.create_texture_from_surface(&cat_sprite).unwrap();
    let mut cat_x = 0;
    let cat_y = (height - cat_img.height()) as i32;
    let mut cat_speed = 0;
    let cat_rect = sdl3::rect::Rect::new(cat_x, cat_y, cat_img.width(), cat_img.height());
    canvas.present();
    let mut events = sdl3_context.event_pump().unwrap();
    'running: loop {
        for event in events.poll_iter() {
            match event {
                sdl3::event::Event::Quit {..} |
                sdl3::event::Event::KeyDown {keycode: Some(sdl3::keyboard::Keycode::Escape), .. } => {
                    break 'running;
                },
                sdl3::event::Event::KeyDown {keycode: Some(sdl3::keyboard::Keycode::Left), ..} |
                sdl3::event::Event::KeyDown {keycode: Some(sdl3::keyboard::Keycode::A), ..} => {
                    cat_speed = -5;
                },
                sdl3::event::Event::KeyDown {keycode: Some(sdl3::keyboard::Keycode::Right), ..} |
                sdl3::event::Event::KeyDown {keycode: Some(sdl3::keyboard::Keycode::D), ..} => {
                    cat_speed = 5;
                },
                sdl3::event::Event::KeyUp {keycode: Some(sdl3::keyboard::Keycode::Left), ..} |
                sdl3::event::Event::KeyUp {keycode: Some(sdl3::keyboard::Keycode::A), ..} |
                sdl3::event::Event::KeyUp {keycode: Some(sdl3::keyboard::Keycode::Right), ..} |
                sdl3::event::Event::KeyUp {keycode: Some(sdl3::keyboard::Keycode::D), ..} => {
                    cat_speed = 0;
                }
                _ => {}
            }
        }
        cat_x += cat_speed;
        if cat_x > width as i32 - cat_img.width() as i32 {
            cat_x = width as i32 - cat_img.width() as i32;
        }
        else if cat_x < 0 {cat_x = 0;}
        canvas.copy(&bg_img, None, None).unwrap();
        canvas.copy(&cat_img, None, cat_rect).unwrap();
        canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 60))
    }
}
