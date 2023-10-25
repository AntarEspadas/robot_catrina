pub mod animator;
pub mod arduino;
pub mod catrina;
pub mod face_tracker;

use std::{error::Error, sync::Arc, thread, time::Duration};

use arduino::Arduino;
use catrina::{Catrina, Pins};
use face_tracker::FaceTracker;

const ADDRESS: &str = "127.0.0.1:11573";
const SERIAL_PORT: &str = "/dev/ttyUSB0";

fn main() -> Result<(), Box<dyn Error>> {
    let arduino = Arduino::new(SERIAL_PORT);

    let catrina = Catrina::new(arduino, 0.7, 0.5, Pins { neck: 4 });
    let catrina = Arc::new(catrina);

    thread::sleep(Duration::from_secs(1));

    catrina.arduino.write(3, 90);

    let catrina_clone = Arc::clone(&catrina);

    thread::spawn(move || {
        let mut tracker = FaceTracker::new(
            ADDRESS,
            |data| catrina_clone.handle_face_tracker_data(data),
            || catrina_clone.handle_face_lost(),
            || catrina_clone.handle_timeout(),
        );
        // tracker.start();
    });

    // catrina.main_loop();

    let duration = Duration::from_secs_f32(0.75);

    loop {
        let catrina1 = Arc::clone(&catrina);
        let catrina2 = Arc::clone(&catrina);

        catrina1
            .arduino
            .animate(5)
            .start_angle(90)
            .to(0, 0.75)
            .sleep(0.75)
            .to(180, 0.75)
            .sleep(0.75)
            .to(90, 0.75)
            .sleep(0.75);

        // let handle1 = thread::spawn(move || {
        // catrina1
        //     .arduino
        //     .animate(5, 90, &[(duration, 0), (duration, 180), (duration, 90)])
        // catrina1.arduino.write_smooth(5, 90, 0, duration);
        // catrina1.arduino.write_smooth(5, 0, 180, duration);
        // catrina1.arduino.write_smooth(5, 180, 90, duration);
        // });

        // let handle2 = thread::spawn(move || {
        //     catrina2
        //         .arduino
        //         .animate(6, 90, &[(duration, 0), (duration * 2, 90)]);
        //     // catrina2.arduino.write_smooth(6, 90, 180, duration);
        //     // catrina2.arduino.write_smooth(6, 180, 45, duration);
        //     // catrina2.arduino.write_smooth(6, 45, 90, duration);
        // });

        // handle1.join().unwrap();
        // handle2.join().unwrap();
    }

    Ok(())
}
