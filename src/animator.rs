use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::arduino::Arduino;

pub struct Animator {
    arduino: Arc<Arduino>,
    pin: u8,
    angle: Mutex<u8>,
}

impl Animator {
    pub fn new(arduino: &Arc<Arduino>, pin: u8) -> Self {
        let arduino = Arc::clone(arduino);
        let start_angle = if pin % 2 == 0 { 0 } else { 180 };
        Self {
            arduino,
            pin,
            angle: Mutex::new(start_angle),
        }
    }

    pub fn angle(&self) -> u8 {
        *self.angle.lock().unwrap()
    }

    pub fn set_smooth(&self, angle: u8, seconds: f32) -> &Self {
        let mut self_angle = self.angle.lock().unwrap();
        self.arduino.write_smooth(
            self.pin,
            *self_angle,
            angle,
            Duration::from_secs_f32(seconds),
        );
        *self_angle = angle;
        self
    }

    pub fn set(&self, angle: u8) -> &Self {
        let mut self_angle = self.angle.lock().unwrap();
        self.arduino.write(self.pin, angle);
        *self_angle = angle;
        self
    }

    pub fn increment(&self, value: u8) -> &Self {
        let mut self_angle = self.angle.lock().unwrap();
        if *self_angle <= 180 - value {
            *self_angle += value;
        } else {
            *self_angle = 180;
        }
        self.arduino.write(self.pin, *self_angle);
        self
    }

    pub fn decrement(&self, value: u8) -> &Self {
        let mut self_angle = self.angle.lock().unwrap();
        if *self_angle >= value {
            *self_angle -= value;
        } else {
            *self_angle = 0;
        }
        self.arduino.write(self.pin, *self_angle);
        self
    }

    pub fn increment_smooth(&self, value: u8, seconds: f32) -> &Self {
        let mut self_angle = self.angle.lock().unwrap();
        let angle = if *self_angle <= 180 - value {
            *self_angle + value
        } else {
            180
        };
        self.arduino.write_smooth(
            self.pin,
            *self_angle,
            angle,
            Duration::from_secs_f32(seconds),
        );
        *self_angle = angle;
        self
    }

    pub fn decrement_smooth(&self, value: u8, seconds: f32) -> &Self {
        let mut self_angle = self.angle.lock().unwrap();
        let angle = if *self_angle >= value {
            *self_angle - value
        } else {
            0
        };
        self.arduino.write_smooth(
            self.pin,
            *self_angle,
            angle,
            Duration::from_secs_f32(seconds),
        );
        *self_angle = angle;
        self
    }

    pub fn sleep(&self, seconds: f32) -> &Self {
        thread::sleep(Duration::from_secs_f32(seconds));
        self
    }
}
