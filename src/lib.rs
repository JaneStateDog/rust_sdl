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
    sdl2_window: sdl2::video::Window,
}

impl Window {
    pub fn new(sdl_context: &sdl2::Sdl, title: String, width: u32, height: u32) -> Window {
        let video_subsystem = sdl_context.video().unwrap();
        let sdl2_window = video_subsystem.window(&title, width, height)
            .position_centered()
            .build()
            .unwrap();

        Window {
            title,
            width,
            height,

            video_subsystem,
            sdl2_window,
        }
    }

    pub fn get_title(&self) -> &String { &self.title }
    pub fn get_width(&self) -> &u32 { &self.width }
    pub fn get_height(&self) -> &u32 { &self.height }
}

// -- SURFACE --
pub struct Surface {

}

impl Surface {
    
}

// -- CANVAS --
pub struct Canvas {

}

impl Canvas {

}