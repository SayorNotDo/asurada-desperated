use crate::event::wake_event::WakeEvent;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Stream,
};
use std::time;
// use tokio::sync::mpsc::Sender;
use crossbeam_channel::Sender;

pub struct AudioStream {
    pub stream: Stream,
}

// async fn process_audio(sender: Sender<WakeEvent>, data: Vec<f32>) {
//     sender
//         .send(WakeEvent::AudioFrame(data))
//         .await
//         .expect("failed to send audio frame");
// }

impl AudioStream {
    pub fn new(
        device: &cpal::Device,
        config: Option<&cpal::StreamConfig>,
        event_sender: Sender<WakeEvent>,
    ) -> Result<Self, anyhow::Error> {
        list_supported_configs(&device);
        // Create audio input stream
        let stream_config = match config {
            Some(config) => config.clone(),
            None => get_compatible_config(device)?,
        };

        let stream = device.build_input_stream(
            &stream_config,
            move |data: &[f32], _| {
                // Send raw audio data to wake word detection module
                event_sender
                    .send(WakeEvent::AudioFrame(data.to_vec()))
                    .unwrap();
            },
            |err| eprintln!("Audio stream error: {:?}", err),
            Some(time::Duration::from_secs(5)),
        )?;

        Ok(Self { stream })
    }

    pub fn start(&self) {
        self.stream.play().unwrap();
    }
}

fn list_supported_configs(device: &cpal::Device) {
    println!("Supported configurations:");
    let configs = device.supported_input_configs().unwrap();
    for config in configs {
        println!("{:?}", config);
    }
}

fn get_compatible_config(device: &cpal::Device) -> Result<cpal::StreamConfig, anyhow::Error> {
    let mut configs = device.supported_input_configs()?;

    // Prefer f32 format, 16kHz sample rate, mono channel
    let preferred_config = configs.find(|c| {
        c.sample_format() == cpal::SampleFormat::F32
            && c.min_sample_rate() <= cpal::SampleRate(44100)
            && c.max_sample_rate() >= cpal::SampleRate(44100)
            && c.channels() == 1
    });

    match preferred_config {
        Some(config) => Ok(config.with_sample_rate(cpal::SampleRate(44100)).into()),
        None => Err(anyhow::anyhow!("No compatible audio configuration found")),
    }
}

#[allow(dead_code)]
#[cfg(test)]
fn test_audio_stream_play() {}
