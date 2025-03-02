// EYE MOVEMENT CONSTRAINTS (measure from animatronics)
pub const MAX_HORIZ_ANGLE: i32 = 45;
pub const MAX_VERT_ANGLE: i32 = 45;
pub const MAX_EYELID_TILT_ANGLE: u32 = 45;

// BLINKING PARAMETERS (tune for desired behavior)
pub const BLINK_DURATION_MS: u64 = 5;  // length of blink
pub const BLINK_INTERVAL_MS: u64 = 1000;  // time between blinks
pub const BLINK_INTERVAL_VARIATION_MS: u64 = 500;  // variation from interval