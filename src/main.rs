mod audio;
mod audio_engine;
mod audio_output;
mod constants;
mod midi;
mod midi_input;

use audio_engine::SynthEngine;
use audio_output::AudioOutput;
use std::sync::{Arc, Mutex};

fn test_audio_init() {
    let synth = Arc::new(Mutex::new(SynthEngine::new(44100.0, 8)));
    match audio_output::AudioOutput::new(synth) {
        Ok(_) => println!("audio output initialized successfully"),
        Err(e) => println!("audio initialization failed: {}", e),
    }
}

fn main() {
    println!("TESTING MIDI Keyboard Connection");
    //test_audio_init();
    let sample_rate = 44100.0;
    let max_voices = 8;

    let synth_engine = Arc::new(Mutex::new(SynthEngine::new(sample_rate, max_voices)));

    let _audio_output = match AudioOutput::new(Arc::clone(&synth_engine)) {
        Ok(audio) => {
            println!("✅ Audio system ready");
            audio
        }
        Err(e) => {
            println!("❌ Failed to initialize audio: {}", e);
            return;
        }
    };

    match midi_input::MidiInputHandler::new() {
        Ok((_handler, receiver)) => {
            println!("MIDI input connected successfully");
            println!("Press keys on your MIDI keyboard (Ctrl + C to exit) ... ");

            loop {
                if let Ok(msg) = receiver.recv() {
                    if let Ok(mut synth) = synth_engine.lock() {
                        synth.handle_midi_message(msg.clone());
                    }
                    match msg {
                        midi_input::MidiMessage::NoteOn { note, velocity } => {
                            let frequency = midi::midi_note_to_frequency(note);
                            println!(
                                "Note On: MIDI {} = {:.2} Hz (velocity {})",
                                note, frequency, velocity
                            );
                        }
                        midi_input::MidiMessage::NoteOff { note, velocity: _ } => {
                            println!("Note OFF: MIDI {}", note);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to MIDI input: {}", e);
            println!("Make sure your MIDI keyboard is connected!");
        }
    }
}
