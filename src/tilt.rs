use crate::eyes::{EyeState, Transformation};

// NOTE I wrote this wrong. Should be rewritten so eyelids tilt with the eye's movement
pub struct TiltEyelids {
    threshold: i32  // between 0 and 100
}

impl TiltEyelids {
    pub fn new(threshold: i32) -> Self {
        TiltEyelids {
            threshold: 20
        }
    }
}

impl Transformation for TiltEyelids {
    fn transform(&mut self, eye_state: &mut EyeState) {
        if eye_state.left_eyelid_gap.abs() > self.threshold {
            let sign = if eye_state.left_eyelid_gap < 0 {-1} else {1};
            eye_state.left_eyelid_gap = self.threshold * sign;
        }

        if eye_state.right_eyelid_gap.abs() > self.threshold {
            let sign = if eye_state.right_eyelid_gap < 0 {-1} else {1};
            eye_state.right_eyelid_gap = self.threshold * sign;
        }
    }
}