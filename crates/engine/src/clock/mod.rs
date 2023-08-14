use std::time;

#[allow(dead_code)]
pub struct Clock {
    pub target_fps: u32,
    pub target_ups: u32,
    fps: f64,
    ups: f64,

    now: time::Instant,
}

impl Clock {
    pub fn new(target_fps: u32, target_ups: u32) -> Self {
        Self {
            target_fps,
            target_ups,
            fps: target_fps as f64, // This is probably not needed and can just be 0.0
            ups: target_ups as f64,

            now: time::Instant::now(),
        }
    }
}