use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender},
    thread,
};

use rand::seq::SliceRandom;

use soloud::*;

enum PlayerMessage {
    Play(PathBuf),
    Stop,
}

pub struct Player {
    sender: Sender<PlayerMessage>,
    base_path: PathBuf,
}

impl Player {
    pub fn new(base_path: PathBuf) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();
        thread::spawn(move || {
            Self::main_loop(receiver);
        });
        Self { sender, base_path }
    }

    pub fn start() {}

    pub fn play(&self, path: PathBuf) {
        self.sender.send(PlayerMessage::Play(path)).unwrap();
    }

    pub fn stop(&self) {
        self.sender.send(PlayerMessage::Stop).unwrap();
    }

    pub fn play_random(&self, folder_path: PathBuf) {
        let folder_path = self.base_path.join(folder_path);

        let mut rng = rand::thread_rng();
        let paths: Vec<_> = std::fs::read_dir(folder_path)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().unwrap() == "wav")
            .collect();
        let path = paths.choose(&mut rng).unwrap().to_owned();
        self.play(path);
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
