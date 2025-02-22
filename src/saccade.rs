use crate::eyes::{EyeState, Transformation};

// Causes the eye to blink if a saccade greater than a threshold is detected
pub struct SaccadeBlink {}

impl Saccades {
    pub fn new() -> Self {
        Saccades {}
    }
}

impl Transformation for Saccades {
    fn transform(&mut self, eye_state: &mut EyeState) {

    }
}