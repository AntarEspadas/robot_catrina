use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::arduino::Arduino;

pub struct Animator {
    arduino: Arc<Arduino>,
    pin: u8,
    value: Mutex<u8>,
}

impl Animator {
    pub fn new(arduino: &Arc<Arduino>, pin: u8) -> Self {
        let arduino = Arc::clone(arduino);
        let start_value = if pin % 2 == 0 { 0 } else { 180 };
        Self {
            arduino,
            pin,
            value: Mutex::new(start_value),
        }
    }

    pub fn value(&self) -> u8 {
        *self.value.lock().unwrap()
    }

    pub fn set_smooth(&self, value: u8, seconds: f32) -> &Self {
        let mut self_value = self.value.lock().unwrap();
        self.arduino.write_smooth(
            self.pin,
            *self_value,
            value,
            Duration::from_secs_f32(seconds),
        );
        *self_value = value;
        self
    }

    pub fn set(&self, value: u8) -> &Self {
        let mut self_value = self.value.lock().unwrap();
        self.arduino.write(self.pin, value);
        *self_value = value;
        self
    }

    pub fn increment(&self, value: u8) -> &Self {
        let mut self_value = self.value.lock().unwrap();
        if *self_value <= 180 - value {
            *self_value += value;
        } else {
            *self_value = 180;
        }
        self.arduino.write(self.pin, *self_value);
        self
    }

    pub fn decrement(&self, value: u8) -> &Self {
        let mut self_value = self.value.lock().unwrap();
        if *self_value >= value {
            *self_value -= value;
        } else {
            *self_value = 0;
        }
        self.arduino.write(self.pin, *self_value);
        self
    }

    pub fn increment_smooth(&self, value: u8, seconds: f32) -> &Self {
        let mut self_value = self.value.lock().unwrap();
        let value = if *self_value <= 180 - value {
            *self_value + value
        } else {
            180
        };
        self.arduino.write_smooth(
            self.pin,
            *self_value,
            value,
            Duration::from_secs_f32(seconds),
        );
        *self_value = value;
        self
    }

    pub fn decrement_smooth(&self, value: u8, seconds: f32) -> &Self {
        let mut self_value = self.value.lock().unwrap();
        let value = if *self_value >= value {
            *self_value - value
        } else {
            0
        };
        self.arduino.write_smooth(
            self.pin,
            *self_value,
            value,
            Duration::from_secs_f32(seconds),
        );
        *self_value = value;
        self
    }

    pub fn sleep(&self, seconds: f32) -> &Self {
        thread::sleep(Duration::from_secs_f32(seconds));
        self
    }
}
