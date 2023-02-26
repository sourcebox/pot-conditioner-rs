//! Unit tests

use super::*;

#[test]
fn settle_12bit_50hz() {
    const SAMPLING_RATE: i32 = 50;
    const ADC_RANGE: (i32, i32) = (0, 4095);
    const OUTPUT_RANGE: (i32, i32) = (0, 4095);

    let mut conditioner = PotConditioner::new(SAMPLING_RATE, ADC_RANGE, OUTPUT_RANGE);

    let input_values = [503, 847, 1050, 1030, 1052, 1049, 1047, 1052, 1050];

    for (tick, input_value) in input_values.into_iter().enumerate() {
        let output_value = conditioner.update(input_value, tick as u64);
        println!("{}", output_value);
    }
}

#[test]
fn settle_12bit_1000hz() {
    const SAMPLING_RATE: i32 = 1000;
    const ADC_RANGE: (i32, i32) = (0, 4095);
    const OUTPUT_RANGE: (i32, i32) = (0, 4095);

    let mut conditioner = PotConditioner::new(SAMPLING_RATE, ADC_RANGE, OUTPUT_RANGE);

    let input_values = [503, 847, 1050, 1030, 1052, 1049, 1047, 1052, 1050];

    for (tick, input_value) in input_values.into_iter().enumerate() {
        let output_value = conditioner.update(input_value, tick as u64);
        println!("{}", output_value);
    }
}
