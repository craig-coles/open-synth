use crate::constants::{DEFAULT_AMPLITUDE, FULL_WAVE_CYCLE_RADIANS};
use crate::midi::{midi_note_to_frequency, A4_FREQUENCY_HZ};
pub struct Oscillator {
    frequency: f32,
    phase: f32,
    sample_rate: f32,
    amplitude: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            frequency: A4_FREQUENCY_HZ,
            phase: 0.0,
            sample_rate,
            amplitude: DEFAULT_AMPLITUDE,
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = (self.phase * FULL_WAVE_CYCLE_RADIANS).sin() * self.amplitude;
        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        sample
    }

    pub fn set_frequency(&mut self, frequency_hz: f32) {
        self.frequency = frequency_hz;
    }

    pub fn set_midi_note(&mut self, midi_note: u8) {
        self.frequency = midi_note_to_frequency(midi_note);
    }
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude.clamp(0.0, 1.0);
    }
    pub fn get_amplitude(&self) -> f32 {
        self.amplitude
    }
}
