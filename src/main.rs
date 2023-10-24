pub mod arduino;
pub mod face_tracker;

use std::{error::Error, thread, time::Duration};

use arduino::Arduino;
use face_tracker::FaceTracker;

const ADDRESS: &str = "127.0.0.1:11573";
const SERIAL_PORT: &str = "/dev/ttyUSB0";

fn main() -> Result<(), Box<dyn Error>> {
    let offset = 0.7;
    let dead_zone = 1.0;

    let arduino = Arduino::new(SERIAL_PORT);

    thread::sleep(Duration::from_secs(1));

    let mut current_angle = 90;
    let increment = 3;
    arduino.write(3, current_angle);

    let mut tracker = FaceTracker::new(
        ADDRESS,
        |data| {
            let x = data.x + offset;
            if x < -dead_zone {
                println!("right");
                if current_angle >= increment {
                    current_angle -= 3;
                    arduino.write(3, current_angle);
                }
            } else if x > dead_zone {
                println!("left");
                if current_angle <= 180 - increment {
                    current_angle += 3;
                    arduino.write(3, current_angle);
                }
            } else {
                println!("center");
                // arduino.write(3, 90);
            }
            println!("x: {x}");
        },
        || {
            // arduino.write(3, 90);
        },
        || println!("Timeout"),
    );

    // vec![
    //     arduino.write_smooth(3, 90, 180, Duration::from_secs_f32(0.75)),
    //     arduino.write_smooth(6, 90, 0, Duration::from_secs_f32(0.75)),
    // ].into_iter().for_each(|t| t.join().unwrap());

    // vec![
    //     arduino.write_smooth(3, 180, 90, Duration::from_secs_f32(0.75)),
    //     arduino.write_smooth(6, 0, 90, Duration::from_secs_f32(0.75)),
    // ].into_iter().for_each(|t| t.join().unwrap());

    tracker.start();
    Ok(())
}
