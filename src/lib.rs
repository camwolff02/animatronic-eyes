mod blink;
mod eyes;
mod tilt;

use blink::Blink;
use tilt::TiltEyelids;
use eyes::EyeState;
use saccade::SaccadeBlink;

fn main() {
    // Define eyes
    let mut eyes = EyeState::new();

    // Define transformations on eyes
    let mut blink = Blink::new();
    let mut tilt = TiltEyelids::new(0.5);
    let mut saccade = SaccadeBlink::new();

    loop {
        // TODO get input here

        // Move the eyes
        eyes.look(0, 0);
        eyes.move_eyelids(1.0, 1.0);

        // Then apply realistic transformations
        let new_eyes = eyes
            .transform(&mut blink)
            .transform(&mut tilt)
            .transform(&mut saccade);

        // TODO apply eye state to servo controller here
    }
}
