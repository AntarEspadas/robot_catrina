pub mod animator;
pub mod arduino;
pub mod catrina;
pub mod face_tracker;
pub mod sound;

use std::{
    error::Error,
    sync::Arc,
    thread::{self, sleep, Thread},
    time::Duration,
};

use arduino::Arduino;
use catrina::{animation::Animation, Catrina, Pins};
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
        leds: 12,
    };

    let player = sound::Player::new("/home/rpi/Music/catrina".into());

    let catrina = Catrina::new(arduino, player, 0.7, 0.5, pins);
    let catrina = Arc::new(catrina);

    thread::sleep(Duration::from_secs(1));

    motion_test(&catrina);

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

    // catrina.main_loop();

    Ok(())
}

fn motion_test(catrina: &Catrina) {
    loop {
        catrina.animators.leds.set(1);
        std::thread::sleep(Duration::from_secs(1));
        catrina.animators.leds.set(0);
        std::thread::sleep(Duration::from_secs(1));
    }
    // Animation::play_parallel(vec![
    //     catrina.raise_left_shoulder(),
    //     catrina.raise_left_elbow(),
    // ]);

    // thread::sleep(Duration::from_secs_f32(0.5));

    // Animation::play_parallel(vec![
    //     catrina.lower_left_shoulder(),
    //     catrina.lower_left_elbow(),
    // ]);
}
