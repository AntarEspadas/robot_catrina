use std::sync::Arc;

use crate::animator::Animator;

enum AnimationFrame {
    SetSmooth(u8, f32),
    Set(u8),
    Increment(u8),
    Decrement(u8),
    IncrementSmooth(u8, f32),
    DecrementSmooth(u8, f32),
    Sleep(f32),
}

pub struct Animation {
    frames: Vec<AnimationFrame>,
    animator: Arc<Animator>,
}

impl Animation {
    pub fn new(animator: &Arc<Animator>) -> Self {
        Self {
            frames: Vec::new(),
            animator: Arc::clone(animator),
        }
    }

    pub fn set_smooth(mut self, angle: u8, seconds: f32) -> Self {
        self.frames.push(AnimationFrame::SetSmooth(angle, seconds));
        self
    }

    pub fn set(mut self, angle: u8) -> Self {
        self.frames.push(AnimationFrame::Set(angle));
        self
    }

    pub fn increment(mut self, value: u8) -> Self {
        self.frames.push(AnimationFrame::Increment(value));
        self
    }

    pub fn decrement(mut self, value: u8) -> Self {
        self.frames.push(AnimationFrame::Decrement(value));
        self
    }

    pub fn increment_smooth(mut self, angle: u8, seconds: f32) -> Self {
        self.frames
            .push(AnimationFrame::IncrementSmooth(angle, seconds));
        self
    }

    pub fn decrement_smooth(mut self, angle: u8, seconds: f32) -> Self {
        self.frames
            .push(AnimationFrame::DecrementSmooth(angle, seconds));
        self
    }

    pub fn sleep(mut self, seconds: f32) -> Self {
        self.frames.push(AnimationFrame::Sleep(seconds));
        self
    }

    pub fn play(&self) {
        for frame in &self.frames {
            match frame {
                AnimationFrame::SetSmooth(angle, seconds) => {
                    self.animator.set_smooth(*angle, *seconds);
                }
                AnimationFrame::Set(angle) => {
                    self.animator.set(*angle);
                }
                AnimationFrame::Increment(value) => {
                    self.animator.increment(*value);
                }
                AnimationFrame::Decrement(value) => {
                    self.animator.decrement(*value);
                }
                AnimationFrame::IncrementSmooth(angle, seconds) => {
                    self.animator.increment_smooth(*angle, *seconds);
                }
                AnimationFrame::DecrementSmooth(angle, seconds) => {
                    self.animator.decrement_smooth(*angle, *seconds);
                }
                AnimationFrame::Sleep(seconds) => {
                    std::thread::sleep(std::time::Duration::from_secs_f32(*seconds));
                }
            }
        }
    }

    pub fn play_parallel(mut animations: Vec<Animation>) {
        if animations.is_empty() {
            return;
        }
        // No need to create a new thread for the last animation
        // we can just use the current thread
        let last = animations.remove(animations.len() - 1);
        let handles: Vec<_> = animations
            .into_iter()
            .map(|animation| {
                std::thread::spawn(move || {
                    animation.play();
                })
            })
            .collect();
        last.play();
        handles.into_iter().for_each(|handle| {
            handle.join().unwrap();
        });
    }
}
