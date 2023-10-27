use super::{animation::Animation, Catrina};

impl Catrina {
    pub fn raise_left_shoulder(&self) -> Animation {
        Animation::new(&self.animators.left_shoulder).set_smooth(135, 0.75)
    }

    pub fn lower_left_shoulder(&self) -> Animation {
        Animation::new(&self.animators.left_shoulder).set_smooth(180, 0.75)
    }

    pub fn raise_left_elbow(&self) -> Animation {
        Animation::new(&self.animators.left_elbow).set_smooth(135, 0.75)
    }

    pub fn lower_left_elbow(&self) -> Animation {
        Animation::new(&self.animators.left_elbow).set_smooth(0, 0.75)
    }
}
