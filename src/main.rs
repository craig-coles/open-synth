mod audio;
mod constants;
mod midi;
mod midi_input;

fn main() {
    println!("TESTING MIDI Keyboard Connection");

    match midi_input::MidiInputHandler::new() {
        Ok((_handler, receiver)) => {
            println!("MIDI input connected successfully");
            println!("Press keys on your MIDI keyboard (Ctrl + C to exit) ... ");

            loop {
                if let Ok(msg) = receiver.recv() {
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
