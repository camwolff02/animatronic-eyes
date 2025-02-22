const MAX_HORIZ_ANGLE: i32 = 90;
const MAX_VERT_ANGLE: i32 = 90;
const MAX_EYELID_TILT_ANGLE: i32 = 90;

pub trait Transformation {
    fn transform(&mut self, eye_state: &mut EyeState);
}

pub struct EyeState {
    horiz_angle: i32,          // centered at 0 degrees
    vert_angle: i32,           // centered at 0 degrees
    eyelid_tilt_angle: i32,    // centered at 0 degrees
    pub left_eyelid_gap: u32,  // closed at 0, open at 100
    pub right_eyelid_gap: u32, // closed at 0, open at 100
}

impl Clone for EyeState {
    fn clone(&self) -> Self {
        EyeState {
            horiz_angle: self.horiz_angle,
            vert_angle: self.vert_angle,
            eyelid_tilt_angle: self.eyelid_tilt_angle,
            left_eyelid_gap: self.left_eyelid_gap,
            right_eyelid_gap: self.right_eyelid_gap,
        }
    }
}

impl EyeState {
    pub fn new() -> Self {
        // Instantiate a new eye staring wide-eyed straight ahead
        EyeState {
            horiz_angle: 0,
            vert_angle: 0,
            eyelid_tilt_angle: 0,
            left_eyelid_gap: 1,
            right_eyelid_gap: 1,
        }
    }

    pub fn look(&mut self, horiz_angle: i32, vert_angle: i32) {
        // Clamp the desired angle's to the eye's actual movement range
        self.horiz_angle = horiz_angle.clamp(-MAX_HORIZ_ANGLE, MAX_HORIZ_ANGLE);
        self.vert_angle = vert_angle.clamp(-MAX_VERT_ANGLE, MAX_VERT_ANGLE);
    }

    // x = forward, y = left, z = up
    pub fn look_at_point(&mut self, x: f32, y: f32, z: f32) {
        // Calculate the arc tangent for each position to get the angle,
        // round to the nearest integer degree, and clamp to the usable
        // range of the eye
        let horiz_angle = (x).atan2(z).round() as i32;
        let vert_angle = (z).atan2(y).round() as i32;
        self.look(horiz_angle, vert_angle);
    }

    pub fn move_eyelids(&mut self, left_eyelid_gap: u32, right_eyelid_gap: u32) {
        // Ensure eyelid gaps are a proportion
        self.left_eyelid_gap = left_eyelid_gap.clamp(0, 100);
        self.right_eyelid_gap = right_eyelid_gap.clamp(0, 100);
    }

    // Defining additional generic transformations on the eye
    // These transformations can be destructive, so we always copy
    pub fn transform<T: Transformation>(&self, transformation: &mut T) -> Self {
        let mut new = self.clone();
        transformation.transform(&mut new);
        new
    }
}
