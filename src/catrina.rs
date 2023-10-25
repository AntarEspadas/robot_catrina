use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{arduino::Arduino, face_tracker::FaceTrackerData};

enum State {
    Idle,
    Dancing,
    FaceLost,
    Tracking { data: FaceTrackerData },
}

struct Angles {
    pub neck: u8,
}

pub struct Pins {
    pub neck: u8,
}

pub struct Catrina {
    pub arduino: Arc<Arduino>,
    x_offset: f32,
    dead_zone: f32,
    pins: Pins,
    angles: Mutex<Angles>,
    state: Mutex<State>,
}

impl Catrina {
    pub fn new(arduino: Arduino, x_offset: f32, dead_zone: f32, pins: Pins) -> Self {
        let arduino = Arc::new(arduino);
        Self {
            arduino,
            x_offset,
            dead_zone,
            pins,
            angles: Mutex::new(Angles { neck: 90 }),
            state: Mutex::new(State::Idle),
        }
    }

    pub fn main_loop(&self) {
        let interval = Duration::from_millis(10);

        loop {
            let state = self.state.lock().unwrap();

            let start = std::time::Instant::now();

            match *state {
                State::Idle => {}
                State::Dancing => {}
                State::FaceLost => {}
                State::Tracking { ref data } => self.move_neck(data),
            }

            drop(state);

            let elapsed = start.elapsed();

            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
        }
    }

    pub fn handle_face_tracker_data(&self, data: &FaceTrackerData) {
        *self.state.lock().unwrap() = State::Tracking { data: data.clone() };
    }

    pub fn handle_timeout(&self) {
        *self.state.lock().unwrap() = State::Idle;
    }

    pub fn handle_face_lost(&self) {
        *self.state.lock().unwrap() = State::FaceLost;
    }

    fn move_neck(&self, data: &FaceTrackerData) {
        let increment = 3;

        let mut angles = self.angles.lock().unwrap();

        let x = data.x + self.x_offset;
        if x < -self.dead_zone {
            if angles.neck >= increment {
                angles.neck -= 3;
                self.arduino.write(self.pins.neck, angles.neck);
            }
        } else if x > self.dead_zone && angles.neck <= 180 - increment {
            angles.neck += 3;
            self.arduino.write(self.pins.neck, angles.neck);
        }
    }
}
