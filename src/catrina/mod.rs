pub mod animation;
pub mod animations;

use std::sync::{Arc, Mutex};

use serde::Deserialize;

use crate::{animator::Animator, arduino::Arduino, face_tracker::FaceTrackerData, sound::Player};

#[derive(Clone)]
enum State {
    Idle { frame: usize },
    FaceLost,
    Tracking { data: FaceTrackerData },
}

pub struct Animators {
    pub neck: Arc<Animator>,
    pub pivot: Arc<Animator>,
    pub left_shoulder: Arc<Animator>,
    pub left_elbow: Arc<Animator>,
    pub left_wrist: Arc<Animator>,
    pub leds: Arc<Animator>,
}

impl Animators {
    fn new(arduino: &Arc<Arduino>, pins: Pins) -> Self {
        Self {
            neck: Arc::new(Animator::new(arduino, pins.neck)),
            pivot: Arc::new(Animator::new(arduino, pins.pivot)),
            left_shoulder: Arc::new(Animator::new(arduino, pins.left_shoulder)),
            left_elbow: Arc::new(Animator::new(arduino, pins.left_elbow)),
            left_wrist: Arc::new(Animator::new(arduino, pins.left_wrist)),
            leds: Arc::new(Animator::new(arduino, pins.leds)),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pins {
    pub neck: u8,
    pub pivot: u8,
    pub left_shoulder: u8,
    pub left_elbow: u8,
    pub left_wrist: u8,
    pub leds: u8,
}

pub struct Catrina {
    pub arduino: Arc<Arduino>,
    pub animators: Animators,
    pub player: Player,
    x_offset: f32,
    dead_zone: f32,
    state: Mutex<State>,
}

impl Catrina {
    pub fn new(
        arduino: Arduino,
        player: Player,
        x_offset: f32,
        dead_zone: f32,
        pins: Pins,
    ) -> Self {
        let arduino = Arc::new(arduino);
        let animators = Animators::new(&arduino, pins);
        Self {
            arduino,
            x_offset,
            dead_zone,
            animators,
            player,
            state: Mutex::new(State::Idle { frame: 0 }),
        }
    }

    pub fn main_loop(&self) {
        self.animators.neck.set_smooth(90, 2.0);

        loop {
            // Clone the state so we don't have to hold the lock
            let state = self.state.lock().unwrap().clone();

            match state {
                State::Idle { frame } => {
                    let frame = self.idle(frame);
                    *self.state.lock().unwrap() = State::Idle { frame };
                }
                State::FaceLost => {}
                State::Tracking { ref data } => self.move_neck(data),
            }
        }
    }

    pub fn handle_face_tracker_data(&self, data: &FaceTrackerData) {
        let mut state = self.state.lock().unwrap();
        if let State::Idle { frame: _ } = *state {
            self.player.play_random("tracking".into());
            self.animators.leds.set(1);
        }
        *state = State::Tracking { data: data.clone() };
    }

    pub fn handle_timeout(&self) {
        *self.state.lock().unwrap() = State::Idle { frame: 0 };
        self.player.play_random("lost".into());
    }

    pub fn handle_face_lost(&self) {
        if let State::Idle { frame: _ } = *self.state.lock().unwrap() {
            return;
        }
        *self.state.lock().unwrap() = State::FaceLost;
        self.animators.leds.set(0);
    }

    fn move_neck(&self, data: &FaceTrackerData) {
        let x = data.x + self.x_offset;

        let increment = 2.0 * x.abs().powf(4.0);
        let increment = increment.min(4.0);

        if x < -self.dead_zone {
            println!("left");
            self.animators.neck.decrement_smooth(increment as u8, 0.1);
        } else if x > self.dead_zone {
            println!("right");
            self.animators.neck.increment_smooth(increment as u8, 0.1);
        }
    }

    fn idle(&self, frame: usize) -> usize {
        match frame {
            0 => {
                self.animators.pivot.set_smooth(0, 2.0);
            }
            1 => {
                self.animators.pivot.set_smooth(180, 2.0);
            }
            _ => (),
        };

        frame + 1
    }
}
