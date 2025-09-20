use crate::constants::OCTAVE_FREQUENCY_MULTIPLIER;

pub const A4_MIDI_NOTE: f32 = 69.0;
pub const A4_FREQUENCY_HZ: f32 = 440.0;
pub const SEMITONES_PER_OCTAVE: f32 = 12.0;
pub const MIDDLE_C_MIDI_NOTE: u8 = 60;
pub const MIN_MIDI_NOTE: u8 = 0;
pub const MAX_MIDI_NOTE: u8 = 127;

pub fn midi_note_to_frequency(midi_note_number: u8) -> f32 {
    let semitone_distance_from_a4 = midi_note_number as f32 - A4_MIDI_NOTE;
    let octave_distance_from_a4 = semitone_distance_from_a4 / SEMITONES_PER_OCTAVE;
    let frequency_multiplier = OCTAVE_FREQUENCY_MULTIPLIER.powf(octave_distance_from_a4);

    A4_FREQUENCY_HZ * frequency_multiplier
}
