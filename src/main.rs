mod blink;
mod eyes;

use blink::Blink;
use eyes::{EyeState, Transformation};

/* TODO MOVE TO OTHER FILE */
struct TiltEyelids {}
impl Transformation for TiltEyelids {
    fn transform(&mut self, eye_state: &mut EyeState) {}
}

fn main() {
    // Define eyes
    let mut eyes = EyeState::new();

    // Define transformations on eyes
    let mut blink = Blink::new();
    let mut tilt = TiltEyelids {};

    loop {
        // TODO get input here

        // Move the eyes
        eyes.look(0, 0);
        eyes.move_eyelids(1.0, 1.0);

        // Then apply realistic transformations
        let new_eyes = eyes
            .transform(&mut blink)
            .transform(&mut tilt);

        // TODO apply eye state to servo controller here
    }
}
