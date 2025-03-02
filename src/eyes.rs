mod constants;
use crate::constants::*;

pub trait Transformation {
    fn transform(&mut self, eye_state: &mut EyeState);
}

pub struct EyeState {
// public
    vert_angle: i32,             // centered at 0 degrees
    left_horiz_angle: i32,       // centered at 0 degrees
    right_horiz_angle: i32,      // centered at 0 degrees

// public mutable
    pub left_eyelid_gap: u32,    // closed at 0, open at 100
    pub right_eyelid_gap: u32,   // closed at 0, open at 100
    pub eyelid_tilt_angle: i32,  // centered at 0 degrees
}

impl Clone for EyeState {
    fn clone(&self) -> Self {
        EyeState {
            left_horiz_angle: self.left_horiz_angle,
            right_horiz_angle: self.right_horiz_angle,
            vert_angle: self.vert_angle,
            left_eyelid_gap: self.left_eyelid_gap,
            right_eyelid_gap: self.right_eyelid_gap,
            eyelid_tilt_angle: self.eyelid_tilt_angle,
        }
    }
}

impl Default for EyeState {
    fn default() -> Self {
        EyeState::new()
    }
}

impl EyeState {
    pub fn new() -> Self {
        // Instantiate a new eye staring wide-eyed straight ahead
        EyeState {
            left_horiz_angle: 0,
            right_horiz_angle: 0,
            vert_angle: 0,
            left_eyelid_gap: 1,
            right_eyelid_gap: 1,
            eyelid_tilt_angle: 0,
        }
    }

    pub fn get_vert_angle(&self) -> i32 {self.vert_angle}
    pub fn get_left_horiz_angle(&self) -> i32 {self.left_horiz_angle}
    pub fn get_right_horiz_angle(&self) -> i32 {self.right_horiz_angle}
    pub fn get_left_eyelid_gap(&self) -> u32 {self.left_eyelid_gap}
    pub fn get_right_eyelid_gap(self) -> u32 {self.right_eyelid_gap}

    pub fn look(&mut self, left_horiz_angle: i32, right_horiz_angle: i32, vert_angle: i32) {
        // Clamp the desired angle's to the eye's actual movement range
        self.left_horiz_angle = left_horiz_angle.clamp(-MAX_HORIZ_ANGLE, MAX_HORIZ_ANGLE);
        self.right_horiz_angle = right_horiz_angle.clamp(-MAX_HORIZ_ANGLE, MAX_HORIZ_ANGLE);
        self.vert_angle = vert_angle.clamp(-MAX_VERT_ANGLE, MAX_VERT_ANGLE);
    }

    // x = forward, y = left, z = up
    pub fn look_at_point(&mut self, x: f32, y: f32, z: f32) {
        // Calculate the arc tangent for each position to get the angle,
        // round to the nearest integer degree, and clamp to the usable
        // range of the eye
        let left_horiz_angle = (x).atan2(z).round() as i32;  // TODO switch to MIDDLE looking, not left eye
        let right_horiz_angle = left_horiz_angle;  // TODO calc right angle
        let vert_angle = (z).atan2(y).round() as i32;
        self.look(left_horiz_angle, right_horiz_angle, vert_angle);
    }

    pub fn move_eyelids(&mut self, left_eyelid_gap: u32, right_eyelid_gap: u32) {
        // Ensure eyelid gaps are a proportion
        self.left_eyelid_gap = left_eyelid_gap.clamp(0, MAX_EYELID_TILT_ANGLE);
        self.right_eyelid_gap = right_eyelid_gap.clamp(0, MAX_EYELID_TILT_ANGLE);
    }

    // Defining additional generic transformations on the eye
    // These transformations can be destructive, so we always copy
    pub fn transform<T: Transformation>(&self, transformation: &mut T) -> Self {
        let mut new = self.clone();
        transformation.transform(&mut new);
        new
    }
}
