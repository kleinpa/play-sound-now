use std::error::Error;
use std::f32::consts::PI;
use std::fmt;

use crate::types::Sample;

/// Generates a sine wave buffer
///
/// # Arguments
///
/// * `frequency` - Frequency of the sine wave (in Hz)
/// * `duration` - Duration of the sine wave (in seconds)
/// * `amplitude` - Amplitude of the sine wave (value between 0.0 and 1.0)
/// * `sample_rate` - Sample rate (samples per second, e.g., 44,100 Hz)
///
/// # Returns
///
/// * `Vec<f32>` - A vector containing the generated sine wave samples
fn generate_sine_wave(frequency: f32, duration: f32, amplitude: f32, sample_rate: u32) -> Vec<f32> {
    // Calculate the total number of samples
    let num_samples = (sample_rate as f32 * duration) as usize;

    // Create a buffer to hold the samples
    let mut buffer = Vec::with_capacity(num_samples);

    // Generate sine wave samples
    for sample_index in 0..num_samples {
        // Calculate the time in seconds for the current sample
        let time = sample_index as f32 / sample_rate as f32;

        // Calculate the sine wave value at the given time
        let sample_value = amplitude * (2.0 * PI * frequency * time).sin();

        // Push the sample value to the buffer
        buffer.push(sample_value);
    }

    buffer
}

pub fn generate() -> Vec<f32> {
    generate_sine_wave(440.0, 2.0, 0.5, 44100)
}

#[cfg(test)]
mod tests {

    use super::generate;

    #[test]
    /// Writes to register[0] should not alter it's value
    fn test_generate() {
        // let mut x = generate();
        // assert_eq!(x, "hello");
    }
}
