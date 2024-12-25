use std::time::{Duration, Instant};

use rand::Rng;

use crate::eyes::{EyeState, Transformation};

/* TODO MOVE TO OTHER FILE */
pub struct Blink {
    duration: Duration,  // ms
    average_interval: Duration,
    interval_variation: Duration,
    _blinking: bool,
    _last_event_time: Instant,
    _next_event_time: Instant,
}


impl Blink {
    fn _get_next_event(average_interval: Duration, interval_variation: Duration) -> Duration {
        // TODO fix the new rand to be created in proper location
        // I need to look into how constants in structs work, and access
        let mut rng = rand::thread_rng();

        // Try to create the interval for the next blink
        let min_ = u64::try_from(
            average_interval.checked_sub(interval_variation)
                .unwrap_or_default()
                .as_millis());

        let max_ = u64::try_from(
            average_interval.checked_add(interval_variation)
                .unwrap_or_default()
                .as_millis());

        // TODO can this be cleaner?
        // Select a random blink time in the next interval, or just the average if
        // a random time cannot be selected
        match min_ {
            Ok(min) => match max_ {
                Ok(max) => Duration::from_millis(rng.gen_range(min..max)),
                Err(_) => Duration::from_millis(u64::try_from(average_interval.as_millis()).unwrap_or_default())
            }
            Err(_) => Duration::from_millis(u64::try_from(average_interval.as_millis()).unwrap_or_default())
        }

    }

    pub fn new() -> Self {
        let average_interval = Duration::from_secs(1);
        let interval_variation = Duration::from_millis(500);
        let now = Instant::now();

        Self {
            duration: Duration::from_millis(10),
            average_interval,
            interval_variation,
            _blinking: false,
            _last_event_time: now,
            _next_event_time: now + Self::_get_next_event(average_interval, interval_variation)
        }
    }

    pub fn from(duration: u64) -> Self {
        let mut blink = Self::new();
        blink.duration = Duration::from_millis(duration);
        blink
    }

}

impl Transformation for Blink {
    // Blink sets eyelids to closed only if a "blink" is occurring
    // Otherwise, transform does nothing
    fn transform(&mut self, eye_state: &mut EyeState) {
        let now = Instant::now();
        
        if self._blinking {
            // if we are blinking, close our eyes
            eye_state.eyelid_separation_dist = 0.;

            // if we have reached the end of our blink, 
            // stop blinking and decide when we'll blink next
            if self._last_event_time - now > self.duration {
                self._blinking = false;
                self._last_event_time = now;
                self._next_event_time = now + Self::_get_next_event(self.average_interval, self.interval_variation)
            } 
        } else if now == self._next_event_time {
            // if it's time to blink, start a blink
            self._blinking = true;
        }
        
    }
}
