use std::{
    thread,
    time,
};

// -- CLOCK --
pub struct Clock {
    dt_nano: f64,
    dt_milli: f64,
    dt_sec: f64,
    fps: f64,

    now: time::Instant,
}

impl Clock {
    pub fn new() -> Clock {
        Clock {
            dt_nano: 0.0,
            dt_milli: 0.0,
            dt_sec: 0.0,
            fps: 0.0,
            now: time::Instant::now(),
        }
    }

    pub fn tick(&mut self, target_fps: u32) {
        thread::sleep(time::Duration::from_nanos(
            (
                (1_000_000_000.0 / target_fps as f64) - self.now.elapsed().as_nanos() as f64
            ) as u64
        ));

        self.dt_nano = self.now.elapsed().as_nanos() as f64;
        self.dt_milli = self.dt_nano / 1_000_000.0;
        self.dt_sec = self.dt_milli / 1_000.0;

        self.fps = 1.0 / self.dt_sec;

        self.now = time::Instant::now();
    }

    pub fn get_dt_nano(&self) -> &f64 { &self.dt_nano }
    pub fn get_dt_milli(&self) -> &f64 { &self.dt_milli }
    pub fn get_dt_sec(&self) -> &f64 { &self.dt_sec }
    pub fn get_fps(&self) -> &f64 { &self.fps }
}

// -- WINDOW --
pub struct Window {
    title: String,
    width: u32,
    height: u32,

    video_subsystem: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,
}

impl Window {
    pub fn new(sdl_context: &sdl2::Sdl, title: &str, width: u32, height: u32) -> Window {
        let video_subsystem = sdl_context.video().unwrap();
        let sdl_window = video_subsystem.window(&title, width, height)
            .position_centered()
            .build()
            .unwrap();

        Window {
            title: String::from(title),
            width,
            height,

            video_subsystem,
            sdl_window,
        }
    }

    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_width(&self) -> &u32 { &self.width }
    pub fn get_height(&self) -> &u32 { &self.height }

    pub fn get_sdl_window(&self) -> &sdl2::video::Window { &self.sdl_window }
}

// -- SURFACE --
pub struct Surface {

}

impl Surface {

}

// -- STATE --
pub struct State {
    sdl_context: sdl2::Sdl,
    events: sdl2::EventPump,
    window: Window,
    clock: Clock,
}

impl State {
    pub fn new() -> State {
        let sdl_context = sdl2::init().unwrap();
        let events = sdl_context.event_pump().unwrap();
        let window = Window::new(&sdl_context, "rust_sdl", 1280, 720);
        let clock = Clock::new();

        State {
            sdl_context,
            events,
            window,
            clock,
        }
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self) {

    }
}