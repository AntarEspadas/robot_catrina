use std::{thread, time::Duration};

use crate::arduino::Arduino;

pub struct Animator<'a> {
    arduino: &'a Arduino,
    pin: u8,
    angle: u8,
}

impl<'a> Animator<'a> {
    pub fn new(arduino: &'a Arduino, pin: u8) -> Self {
        Self {
            arduino,
            pin,
            angle: 0,
        }
    }

    pub fn start_angle(&mut self, angle: u8) -> &mut Self {
        self.angle = angle;
        self
    }

    pub fn to(&mut self, angle: u8, seconds: f32) -> &mut Self {
        self.arduino.write_smooth(
            self.pin,
            self.angle,
            angle,
            Duration::from_secs_f32(seconds),
        );
        self.angle = angle;
        self
    }

    pub fn sleep(&mut self, seconds: f32) -> &mut Self {
        thread::sleep(Duration::from_secs_f32(seconds));
        self
    }
}
