#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

use backlash::Backlash;
use dyn_smooth::{DynamicSmootherEcoI32, I32_FRAC_BITS};

/// Base frequency for dynamic smoother.
const SMOOTHER_BASEFREQ: i32 = (0.1 * (1 << I32_FRAC_BITS) as f32) as i32;

/// Sensitivity of dynamic smoother.
const SMOOTHER_SENSITIVITY: i32 = (0.02 * (1 << I32_FRAC_BITS) as f32) as i32;

/// Default movement threshold.
const MOVEMENT_THRESHOLD_DEFAULT: i32 = 30;

/// Processor struct containing variables and states.
#[derive(Debug)]
pub struct PotConditioner {
    /// Current value.
    value: i32,

    /// Tuple of input value range (min, max).
    input_range: (i32, i32),

    /// Tuple of output value range (min, max).
    output_range: (i32, i32),

    /// Dynamic smoother.
    smoother: DynamicSmootherEcoI32,

    /// Backlash deadband width divided by 2.
    deadband_half_width: i32,

    /// Backlash processor.
    backlash: Backlash<i32>,

    /// Delta between current value and last one.
    delta: i32,

    /// Velocity of movement. Used for movement detection.
    velocity: i32,

    /// Movement threshold.
    movement_threshold: i32,

    /// Moved flag, set in `update()` when threshold was reached.
    moved: bool,

    /// Tick number of last detected movement.
    last_movement: Option<u64>,
}

impl PotConditioner {
    /// Create a new instance.
    /// - `sampling_rate`: sampling rate in Hz.
    /// - `input_range`: tuple of (lowest/highest) input value.
    /// - `input_range`: tuple of (lowest/highest) output value.
    pub fn new(sampling_rate: i32, input_range: (i32, i32), output_range: (i32, i32)) -> Self {
        let deadband_width = (input_range.1 - input_range.0) / 512;

        Self {
            value: 0,
            input_range,
            output_range,
            smoother: DynamicSmootherEcoI32::new(
                SMOOTHER_BASEFREQ,
                sampling_rate << I32_FRAC_BITS,
                SMOOTHER_SENSITIVITY,
            ),
            deadband_half_width: deadband_width / 2,
            backlash: Backlash::new(deadband_width),
            delta: 0,
            velocity: 0,
            movement_threshold: MOVEMENT_THRESHOLD_DEFAULT,
            moved: false,
            last_movement: None,
        }
    }

    /// Sets a new movement threshold. Recommended range is 10-255.
    pub fn set_movement_threshold(&mut self, threshold: i32) {
        self.movement_threshold = threshold;
    }

    /// Update value with new input and return processed value.
    pub fn update(&mut self, value: i32, tick: u64) -> i32 {
        let value = self.smoother.tick(value);
        let value = self.backlash.update(value);
        let value = rescale_and_clamp(
            value,
            self.input_range.0 + self.deadband_half_width,
            self.input_range.1 - self.deadband_half_width,
            self.output_range.0,
            self.output_range.1,
        );

        self.delta = value - self.value;
        self.velocity = (self.velocity + (self.smoother.g() << 8) / self.smoother.g0().max(1)) / 2;

        self.moved = self.delta() != 0
            && (self.velocity() > self.movement_threshold
                || self.value() == self.output_range.0
                || self.value() == self.output_range.1);

        if self.moved {
            self.last_movement = Some(tick);
        }

        self.value = value;

        self.value
    }

    /// Return current value.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Return delta.
    pub fn delta(&self) -> i32 {
        self.delta
    }

    /// Return velocity.
    pub fn velocity(&self) -> i32 {
        self.velocity >> 8
    }

    /// Return if the pot has been moved faster than the given threshold.
    pub fn moved(&self) -> bool {
        self.moved
    }

    /// Return tick number of last detected movement.
    pub fn last_movement(&self) -> Option<u64> {
        self.last_movement
    }

    /// Sets a new output range.
    pub fn set_output_range(&mut self, out_min: i32, out_max: i32) {
        self.output_range = (out_min, out_max);
    }

    /// Returns the current output range.
    pub fn output_range(&self) -> (i32, i32) {
        self.output_range
    }
}

/// Rescale a value to a new range with limiting.
fn rescale_and_clamp(value: i32, in_min: i32, in_max: i32, out_min: i32, out_max: i32) -> i32 {
    ((value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min).clamp(out_min, out_max)
}

#[cfg(test)]
mod tests;
