use std::{thread::{self, JoinHandle}, time::Duration, sync::{Arc, Mutex}};

use serialport::SerialPort;

pub struct Arduino {
    port: Mutex<Box<dyn SerialPort>>,
}

impl Arduino {
    pub fn new(port: &str) -> Arc<Self> {
        let serial_port = serialport::new(port, 9600)
            .timeout(std::time::Duration::from_millis(1000))
            .open()
            .expect("Unable to open serial port");
        let serial_port = Mutex::new(serial_port);
        Arc::new(Self { port: serial_port })
    }

    pub fn write(&self, pin: u8, angle: u8) -> Duration {
        let out_buf = [pin, angle];
        let mut port = self.port.lock().unwrap();
        let start = std::time::Instant::now();
        port
            .write_all(&out_buf)
            .expect("Unable to write to serial port");
        let mut in_buf = [0; 1];
        // Wait for confirmation message
        port
            .read_exact(&mut in_buf)
            .expect("Unable to read from serial port");
        start.elapsed()
    }

    pub fn write_smooth(self: &Arc<Arduino>, pin: u8, start: u8, end: u8, duration: Duration) -> JoinHandle<()> {
        let steps = (duration.as_secs_f32() * 100.0) as i32;
        let increment = (end as f32 - start as f32) / steps as f32;
        let wait_interval = Duration::from_millis(10);
        let arduino = Arc::clone(self);
        thread::spawn(move || {
            for i in 0..steps {
                let angle = start as f32 + increment * i as f32;
                let angle = angle as u8;
                let time_taken = arduino.write(pin, angle);
                if time_taken < wait_interval {
                    thread::sleep(wait_interval - time_taken);
                }
            }
        })
    }
}
