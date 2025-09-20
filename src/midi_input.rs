use midir::{Ignore, MidiInput, MidiInputConnection};
use std::sync::mpsc;

#[derive(Debug, Clone)]
pub enum MidiMessage {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8, velocity: u8 },
}

pub struct MidiInputHandler {
    _connection: MidiInputConnection<()>,
}

impl MidiInputHandler {
    pub fn new() -> Result<(Self, mpsc::Receiver<MidiMessage>), Box<dyn std::error::Error>> {
        let mut midi_input = MidiInput::new("Rust_Synth")?;
        midi_input.ignore(Ignore::None);

        let midi_input_ports = midi_input.ports();
        let input_port = midi_input_ports.first().ok_or("No MIDI input port found")?;
        let port_name = midi_input.port_name(input_port)?;
        println!("Connecting to MIDI input: {}", port_name);
        let (sender, receiver) = mpsc::channel();

        let connection = midi_input.connect(
            input_port,
            "rust-synth-input",
            move |_timestamp, message, _| {
                if let Ok(parsed_midi_message) = parse_midi_message(message) {
                    let _ = sender.send(parsed_midi_message);
                }
            },
            (),
        )?;

        Ok((
            Self {
                _connection: connection,
            },
            receiver,
        ))
    }
}

#[allow(dead_code)]
fn parse_midi_message(message: &[u8]) -> Result<MidiMessage, String> {
    if message.len() < 3 {
        return Err("MIDI message too short".to_string());
    }

    let status_byte = message[0];
    let note_number = message[1];
    let velocity = message[2];
    let message_type = status_byte & 0xF0;

    get_midi_message(note_number, velocity, message_type)
}

fn get_midi_message(
    note_number: u8,
    velocity: u8,
    message_type: u8,
) -> Result<MidiMessage, String> {
    match message_type {
        0x90 if velocity > 0 => Ok(MidiMessage::NoteOn {
            note: note_number,
            velocity,
        }),
        0x90 | 0x80 => Ok(MidiMessage::NoteOff {
            note: note_number,
            velocity,
        }),
        _ => Err(format!(
            "Unsupported MIDI message type: 0x{:02X}",
            message_type
        )),
    }
}
