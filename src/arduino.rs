use std::{
    sync::Mutex,
    thread::{self, sleep},
    time::Duration,
};

use serialport::SerialPort;

pub struct Arduino {
    port: Mutex<Box<dyn SerialPort>>,
}

impl Arduino {
    pub fn connect(port: &str) -> Self {
        println!("Intentando conectar al puerto serial {port}...");
        let serial_port = loop {
            let result = serialport::new(port, 9600)
                .timeout(std::time::Duration::from_secs(2))
                .open();
            match result {
                Ok(port) => {
                    println!("Conectado!");
                    break port;
                }
                // Err(err) => println!(
                //     "Error: {}.\nUnable to connect to serial port {port}, retrying...",
                //     err.description
                // ),
                Err(err) => {
                    println!("Error: {}", err.description);
                    if let serialport::ErrorKind::Io(std::io::ErrorKind::NotFound) = err.kind {
                        println!("Asegúrese de que el Arduino está conectado y su puerto configurado correctamente en config.json (ej. /dev/ttyUSB0, /dev/ttyUSB1, /dev/ttyACM0, etc.)")
                    }
                }
            }
            println!("Reintentando conectar al puerto {port}...");
            sleep(Duration::from_secs(1));
        };
        let serial_port = Mutex::new(serial_port);
        Self { port: serial_port }
    }

    pub fn write(&self, pin: u8, value: u8) -> Duration {
        let out_buf = [pin, value];
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
            let value = start as f32 + increment * i as f32;
            let value = value as u8;
            let time_taken = self.write(pin, value);
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
}
