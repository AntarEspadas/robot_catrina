pub mod animator;
pub mod arduino;
pub mod catrina;
pub mod config;
pub mod face_tracker;
pub mod sound;

use std::{
    error::Error,
    sync::Arc,
    thread::{self},
    time::Duration,
};

use arduino::Arduino;
use catrina::Catrina;
use face_tracker::FaceTracker;

use crate::config::read_config_from_file;

fn main() -> Result<(), Box<dyn Error>> {
    let config = read_config_from_file("./config.json")?;

    let arduino = Arduino::connect(&config.arduino_serial_port);

    let player = sound::Player::create(config.audio_folder);

    let catrina = Catrina::new(arduino, player, 0.7, 0.5, config.pins.clone());
    let catrina = Arc::new(catrina);

    thread::sleep(Duration::from_secs(1));

    motion_test(&catrina);

    let catrina_clone = Arc::clone(&catrina);

    thread::spawn(move || {
        let mut tracker = FaceTracker::new(
            &config.open_see_face_address,
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
