use engine::{
    Engine,
    Window,
    Vector2,
    Component,
};

use engine_derive::Component;

//const TARGET_FPS: u32 = 60;

#[derive(Component)]
pub struct Player {
    move_speed: i32,
}

#[derive(Component)]
pub struct Position {
    pos: Vector2,
}

pub fn move_player(player: Player, pos: Position) {
    println!("test");
}

pub fn main() {
    let mut engine = Engine::new();
    //engine.add_systems(&[move_player]);

    let window = Window::new(&mut engine, "Test", Vector2(1280, 720));
    engine.add_window(window);

    let test = Position { pos: Vector2(2, 2) };
    println!("{}", test.get_name());

    /*engine.spawn(vec![
        &Player { move_speed: 500 },
        &Position { pos: Vector2(50, 50) },
    ]);*/

    //let params = get_parameter_types!(move_player);
    //println!("{:?}", params);
    //println!("woo");

    //let engine = Engine::new();
    //let window = Window::new(&engine, "test", test_vec_3);

    /*
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust_sdl2", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    //let win = Window::new(&sdl_context, "test", 10, 10);
    //let mut s = Surface::new(512, 512, sdl2::pixels::PixelFormatEnum::RGB24).unwrap();
    //let mut c: sdl2::render::Canvas<Surface> = sdl2::render::Canvas::from_surface(s).unwrap();
    
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();

    let mut x = 100;
    let mut y = 100;
    let speed: f64 = 500.0;

    let mut clock = Clock::new();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let keys = event_pump.keyboard_state();
        if keys.is_scancode_pressed(Scancode::A) { x -= (speed * clock.get_dt_sec()) as i32; }
        if keys.is_scancode_pressed(Scancode::D) { x += (speed * clock.get_dt_sec()) as i32; }
        if keys.is_scancode_pressed(Scancode::W) { y -= (speed * clock.get_dt_sec()) as i32; }
        if keys.is_scancode_pressed(Scancode::S) { y += (speed * clock.get_dt_sec()) as i32; }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rect(Rect::new(x, y, 32, 32)).ok();

        canvas.present();

        clock.tick(TARGET_FPS);
        println!("{}", clock.get_dt_milli());
    }
    */
}