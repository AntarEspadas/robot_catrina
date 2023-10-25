use std::{
    sync::Mutex,
    thread::{self},
    time::Duration,
};

use serialport::SerialPort;

use crate::animator::Animator;

pub struct Arduino {
    port: Mutex<Box<dyn SerialPort>>,
}

impl Arduino {
    pub fn new(port: &str) -> Self {
        let serial_port = serialport::new(port, 9600)
            .timeout(std::time::Duration::from_millis(1000))
            .open()
            .expect("Unable to open serial port");
        let serial_port = Mutex::new(serial_port);
        Self { port: serial_port }
    }

    pub fn write(&self, pin: u8, angle: u8) -> Duration {
        let out_buf = [pin, angle];
        let mut port = self.port.lock().unwrap();
        let start = std::time::Instant::now();
        port.write_all(&out_buf)
            .expect("Unable to write to serial port");
        let mut in_buf = [0; 1];
        // Wait for confirmation message
        port.read_exact(&mut in_buf)
            .expect("Unable to read from serial port");
        start.elapsed()
    }

    pub fn write_smooth(&self, pin: u8, start: u8, end: u8, duration: Duration) {
        let steps = (duration.as_secs_f32() * 100.0) as i32;
        let increment = (end as f32 - start as f32) / steps as f32;
        let wait_interval = Duration::from_millis(10);
        for i in 0..steps {
            let angle = start as f32 + increment * i as f32;
            let angle = angle as u8;
            let time_taken = self.write(pin, angle);
            if time_taken < wait_interval {
                thread::sleep(wait_interval - time_taken);
            }
        }
    }

    // pub fn animate(&self, pin: u8, mut start: u8, angles: &[(Duration, u8)]) {
    //     for (duration, angle) in angles {
    //         thread::sleep(*duration);
    //         self.write_smooth(pin, start, *angle, *duration);
    //         start = *angle;
    //     }
    // }

    pub fn animate(&self, pin: u8) -> Animator {
        Animator::new(self, pin)
    }
}
