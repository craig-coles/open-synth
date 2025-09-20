use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::sync::{Arc, Mutex};

use crate::audio::Oscillator;
use crate::midi_input::MidiMessage;

pub struct SynthEngine {
    oscillators: Vec<Oscillator>,
    sample_rate: f32,
}

impl SynthEngine {
    pub fn new(sample_rate: f32, max_voices: usize) -> Self {
        let mut oscillators = Vec::new();
        for _ in 0..max_voices {
            oscillators.push(Oscillator::new(sample_rate));
        }

        Self {
            oscillators,
            sample_rate,
        }
    }

    pub fn handle_midi_message(&mut self, message: MidiMessage) {
        match message {
            MidiMessage::NoteOn { note, velocity } => {
                self.note_on(note, velocity);
            }
            MidiMessage::NoteOff { note, velocity: _ } => {
                self.note_off(note);
            }
        }
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut mixed_sample = 0.0;
        let mut active_voices = 0;

        for oscillator in &mut self.oscillators {
            if oscillator.get_amplitude() > 0.0 {
                mixed_sample += oscillator.next_sample();
                active_voices += 1;
            }
        }

        if active_voices > 0 {
            mixed_sample / active_voices as f32
        } else {
            0.0
        }
    }

    fn note_on(&mut self, note: u8, velocity: u8) {
        if let Some(oscillator) = self
            .oscillators
            .iter_mut()
            .find(|osc| osc.get_amplitude() <= 0.0)
        {
            oscillator.set_midi_note(note);
            let amplitude = (velocity as f32 / 127.0) * 0.3;
            oscillator.set_amplitude(amplitude);
        }
    }

    fn note_off(&mut self, _note: u8) {
        for oscillator in &mut self.oscillators {
            oscillator.set_amplitude(0.0);
        }
    }

    fn is_oscillator_active(&self, oscillator: &Oscillator) -> bool {
        oscillator.get_amplitude() > 0.0
    }
}
