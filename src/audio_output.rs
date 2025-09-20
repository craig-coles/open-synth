use core::error;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig, SupportedStreamConfig};
use std::sync::{Arc, Mutex};

use crate::audio_engine::SynthEngine;

pub struct AudioOutput {
    _stream: Stream,
}
impl AudioOutput {
    pub fn new(synth_engine: Arc<Mutex<SynthEngine>>) -> Result<Self, Box<dyn std::error::Error>> {
        println!("initializing the audio output ... ");

        let host = cpal::default_host();
        println!("audio host: {}", host.id().name());

        let device = host
            .default_output_device()
            .ok_or("no output device available")?;
        println!("output device:{}", device.name()?);

        let config = device.default_output_config()?;

        println!("sample rate: {}hz", config.sample_rate().0);
        println!("channels:{}", config.channels());
        println!("buffer size: {:?}", config.buffer_size());

        let stream = Self::build_output_stream(&device, &config.into(), synth_engine)?;
        stream.play()?;
        println!("audio stream started successfully");
        Ok(AudioOutput { _stream: stream })
    }

    fn build_output_stream(
        device: &Device,
        config: &StreamConfig,
        synth_engine: Arc<Mutex<SynthEngine>>,
    ) -> Result<Stream, Box<dyn std::error::Error>> {
        println!("🔧 Building audio stream...");
        let channels = config.channels as usize;
        println!("🔧 Configuring for {} channels", channels);
        let stream = device.build_output_stream(
            config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                Self::write_audio_data(data, channels, &synth_engine);
            },
            |err| eprintln!("🚨 audio stream Error: {}", err),
            None,
        )?;
        println!("✅ Audio stream built successfully");
        Ok(stream)
    }

    fn write_audio_data(
        output: &mut [f32],
        channels: usize,
        synth_engine: &Arc<Mutex<SynthEngine>>,
    ) {
        static mut FRAME_COUNT: u64 = 0;

        if let Ok(mut engine) = synth_engine.try_lock() {
            for frame in output.chunks_mut(channels) {
                let sample = engine.next_sample();
                for channel_sample in frame.iter_mut() {
                    *channel_sample = sample;
                }
            }

            unsafe {
                FRAME_COUNT += output.len() as u64 / channels as u64;
                if FRAME_COUNT % 44100 == 0 {
                    println!("🎵 Audio running... {} frames processed", FRAME_COUNT);
                }
            }
        } else {
            for sample in output.iter_mut() {
                *sample = 0.0;
            }
        }
    }
}
