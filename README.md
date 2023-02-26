# pot-conditioner

`no_std` Rust crate to improve analog potentiometer readouts connected to an ADC on embedded systems.

## About

Using potentiometers as parameter control is a common task. Because of several causes of electrical and mechanical instabilities, the readouts from an ADC are typically not stable and vary to a certain degree over time. This can lead to all kind of problems, especially when generating messages in an event-based architecture. This crate is designed to improve the readouts by conditioning the ADC input signal in multiple ways:

- Adaptive noise filtering via the [dyn-smooth](https://crates.io/crates/dyn-smooth) crate.
- Compensating mechanical instability via the [backlash](https://crates.io/crates/backlash) crate.
- Movement detection.
- Value scaling.

## Usage Example

This example shows a setup using a typical 12-bit built-in ADC as is available in various MCUs. Depending on the individual components and noise environment, the achievable results may differ.

```rust
use pot_conditioner::PotConditioner;

// Sampling rate in Hz. Should be in range 50-1000Hz.
const SAMPLING_RATE: i32 = 100;

// Minimum and maximum values from ADC. It is recommended to use some margin here.
const ADC_RANGE: (i32, i32) = (10, 4085);

// Minimum and maximum output values.
const OUTPUT_RANGE: (i32, i32) = (0, 1023);

/// Threshold for movement detection. Should be as low as possible, but high
/// enough to prevent accidential detection.
const MOVEMENT_THRESHOLD: i32 = 30;

// Create an instance of the conditioner.
let mut pot = PotConditioner::new(SAMPLING_RATE, ADC_RANGE, OUTPUT_RANGE);

// Set the threshold for the movement detection.
pot.set_movement_threshold(MOVEMENT_THRESHOLD);

// Some dummy input values. In reality, read them from the ADC.
let input_values = [503, 847, 1050, 1030, 1052, 1049, 1047, 1052, 1050];

for (tick, input_value) in input_values.into_iter().enumerate() {
    let output_value = pot.update(input_value, tick as u64);
    println!("{}", output_value);

    // Movement is detected when the threshold is reached.
    if pot.moved() {
        println!("Movement detected.");
    }
}
```

Further recommendations:

- If your ADC supports internal oversampling, enable this feature to get a less noisy input signal.
- Use a dynamic time-based detection threshold to make the pot less sensitive when it's not been used for a while. This can also improve immunity to long-term drift e.g. caused by temperature changes.

## Tests

Run `cargo test` for the unit tests. Use the `--nocapture` flag to show the test output.

## License

Published under the MIT license. Any contribution to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
