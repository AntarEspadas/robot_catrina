pub mod animator;
pub mod arduino;
pub mod catrina;
pub mod face_tracker;
pub mod sound;

use std::{error::Error, sync::Arc, thread, time::Duration};

use arduino::Arduino;
use catrina::{Catrina, Pins};
use face_tracker::FaceTracker;

const ADDRESS: &str = "127.0.0.1:11573";
const SERIAL_PORT: &str = "/dev/ttyUSB0";

fn main() -> Result<(), Box<dyn Error>> {
    let arduino = Arduino::new(SERIAL_PORT);

    let pins = Pins {
        neck: 9,
        pivot: 5,
        left_shoulder: 3,
        left_elbow: 4,
        left_wrist: 8,
    };

    let player = sound::Player::new("/home/rpi/Music/catrina".into());

    let catrina = Catrina::new(arduino, player, 0.7, 0.5, pins);
    let catrina = Arc::new(catrina);

    thread::sleep(Duration::from_secs(1));

    catrina.animators.left_shoulder.set_smooth(90, 1.0);

    let catrina_clone = Arc::clone(&catrina);

    thread::spawn(move || {
        let mut tracker = FaceTracker::new(
            ADDRESS,
            |data| catrina_clone.handle_face_tracker_data(data),
            || catrina_clone.handle_face_lost(),
            || catrina_clone.handle_timeout(),
        );
        tracker.start();
    });

    catrina.main_loop();

    Ok(())
}

fn motion_test(catrina: &Arc<Catrina>) {
    let catrina1 = Arc::clone(catrina);
    let catrina2 = Arc::clone(catrina);

    let handle1 = thread::spawn(move || {
        catrina1
            .animators
            .left_shoulder
            .set_smooth(135, 0.75)
            .sleep(0.5)
            .set_smooth(180, 0.75);
    });

    let handle2 = thread::spawn(move || {
        catrina2
            .animators
            .left_elbow
            .set_smooth(135, 0.75)
            .sleep(0.5)
            .set_smooth(0, 0.75);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
