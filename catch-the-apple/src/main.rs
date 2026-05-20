use rand::{Rng, RngExt};
fn main() {
    let mut random = rand::rng();
    let (width, height): (u32, u32) = (960, 720);
    let sdl3_context = sdl3::init().map_err(|e| e.to_string()).unwrap();
    let video_subsystem = sdl3_context.video().map_err(|e| e.to_string()).unwrap();
    let font_loader = sdl3::ttf::init().map_err(|e| e.to_string()).unwrap();
    let font = font_loader
        .load_font("assets/ARIAL.TTF", 24f32)
        .map_err(|e| e.to_string())
        .unwrap();
    let window = video_subsystem
        .window("Catch the apple", width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();
    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    use sdl3::image::LoadSurface;
    let main_surface = sdl3::surface::Surface::from_file("assets/bg_image.png").unwrap();
    let cat_sprite = sdl3::surface::Surface::from_file("assets/spr_cat.png").unwrap();
    let apple_sprite = sdl3::surface::Surface::from_file("assets/spr_apple.png").unwrap();
    let bg_img = texture_creator
        .create_texture_from_surface(&main_surface)
        .unwrap();
    let cat_img = texture_creator
        .create_texture_from_surface(&cat_sprite)
        .unwrap();
    let apple_img = texture_creator
        .create_texture_from_surface(&apple_sprite)
        .unwrap();
    let mut cat_x = 0;
    let cat_y = (height - cat_img.height()) as i32;
    let mut cat_speed = 0;
    let mut apple_x = random.random_range(0..=width as i32 - apple_img.width() as i32);
    let mut apple_y = 0;
    let apple_speed = 5;
    let mut can_collide = true;
    let mut game_over = false;
    canvas.present();
    let mut events = sdl3_context.event_pump().unwrap();
    let mut score = 0;
    let mut time = 120 * 60;
    'running: loop {
        for event in events.poll_iter() {
            match event {
                sdl3::event::Event::Quit { .. }
                | sdl3::event::Event::KeyDown {
                    keycode: Some(sdl3::keyboard::Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                sdl3::event::Event::KeyDown {
                    keycode: Some(sdl3::keyboard::Keycode::Left),
                    ..
                }
                | sdl3::event::Event::KeyDown {
                    keycode: Some(sdl3::keyboard::Keycode::A),
                    ..
                } => {
                    cat_speed = -5;
                }
                sdl3::event::Event::KeyDown {
                    keycode: Some(sdl3::keyboard::Keycode::Right),
                    ..
                }
                | sdl3::event::Event::KeyDown {
                    keycode: Some(sdl3::keyboard::Keycode::D),
                    ..
                } => {
                    cat_speed = 5;
                }
                sdl3::event::Event::KeyUp {
                    keycode: Some(sdl3::keyboard::Keycode::Left),
                    ..
                }
                | sdl3::event::Event::KeyUp {
                    keycode: Some(sdl3::keyboard::Keycode::A),
                    ..
                }
                | sdl3::event::Event::KeyUp {
                    keycode: Some(sdl3::keyboard::Keycode::Right),
                    ..
                }
                | sdl3::event::Event::KeyUp {
                    keycode: Some(sdl3::keyboard::Keycode::D),
                    ..
                } => {
                    cat_speed = 0;
                }
                _ => {}
            }
        }
        if !game_over {
            let cat_rect = sdl3::rect::Rect::new(cat_x, cat_y, cat_img.width(), cat_img.height());
            let apple_rect =
                sdl3::rect::Rect::new(apple_x, apple_y, apple_img.width(), apple_img.height());
            cat_x += cat_speed;
            apple_y += apple_speed;
            let dx = cat_x as f64 - apple_x as f64;
            let dy = cat_y as f64 - apple_y as f64;
            let distance = (dx * dx + dy * dy).sqrt();
            time -= 1;
            if cat_x > width as i32 - cat_img.width() as i32 {
                cat_x = width as i32 - cat_img.width() as i32;
            } else if cat_x < 0 {
                cat_x = 0;
            }
            if apple_y > height as i32 {
                apple_y = 0;
                apple_x = random.random_range(0..=width as i32 - apple_img.width() as i32);
            }
            if can_collide && distance < 60f64 {
                score += 1;
                apple_y = 0;
                apple_x = random.random_range(0..=width as i32 - apple_img.width() as i32);
                can_collide = false;
            }
            if apple_y > 200 {
                can_collide = true;
            }
            if time == 0 {
                game_over = true;
            }
            canvas.copy(&bg_img, None, None).unwrap();
            if !game_over {
                canvas.copy(&cat_img, None, cat_rect).unwrap();
                canvas.copy(&apple_img, None, apple_rect).unwrap();
            }
            let score_render = font
                .render(&format!("Score: {}", score))
                .solid(sdl3::pixels::Color::WHITE)
                .unwrap();
            let time_render = font
                .render(&format!("Time: {:.0}", time / 60))
                .solid(sdl3::pixels::Color::WHITE)
                .unwrap();
            let score_texture = texture_creator
                .create_texture_from_surface(&score_render)
                .unwrap();
            let time_texture = texture_creator
                .create_texture_from_surface(&time_render)
                .unwrap();
            canvas
                .copy(
                    &score_texture,
                    None,
                    sdl3::rect::Rect::new(0, 0, score_texture.width(), score_texture.height()),
                )
                .unwrap();
            canvas
                .copy(
                    &time_texture,
                    None,
                    sdl3::rect::Rect::new(
                        0,
                        score_texture.height() as i32,
                        time_texture.width(),
                        time_texture.height(),
                    ),
                )
                .unwrap();
            canvas.present();
            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000 / 60))
        }
    }
}
