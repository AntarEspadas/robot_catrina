use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
    thread,
};

use soloud::*;

enum PlayerMessage {
    Play(PathBuf),
    Stop,
}

pub struct Player {
    sender: Sender<PlayerMessage>,
}

impl Player {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        thread::spawn(move || {
            Self::main_loop(receiver);
        });
        Self { sender }
    }

    pub fn start() {}

    pub fn play(&self, path: PathBuf) {
        self.sender.send(PlayerMessage::Play(path)).unwrap();
    }

    pub fn stop(&self) {
        self.sender.send(PlayerMessage::Stop).unwrap();
    }

    fn main_loop(receiver: Receiver<PlayerMessage>) {
        let sl = Soloud::default().unwrap();

        let mut wav = audio::Wav::default();

        loop {
            let message = receiver.recv().unwrap();

            match message {
                PlayerMessage::Play(path) => {
                    println!("Playing sound from path: {:?}", path);
                    wav.load(path).unwrap();
                    sl.play(&wav);
                }
                PlayerMessage::Stop => {
                    sl.stop_all();
                }
            }
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
