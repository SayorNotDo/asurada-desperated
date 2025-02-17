use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Stream,
};
use event::wake_event::WakeEvent;
use std::{sync::mpsc::Sender, time};

pub struct AudioStream {
    pub stream: Stream,
}

impl AudioStream {
    pub fn new(
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        event_sender: Sender<WakeEvent>,
    ) -> Result<Self, anyhow::Error> {
        // 创建音频输入流
        let stream = device.build_input_stream(
            config,
            move |data: &[f32], _| {
                event_sender
                    .send(WakeEvent::AudioFrame(data.to_vec()))
                    .unwrap();
            },
            |err| eprintln!("AudioFrame building error: {:?}", err),
            Some(time::Duration::from_secs(5)),
        )?;

        Ok(Self { stream })
    }

    pub fn start(&self) {
        self.stream.play().unwrap();
    }
}

#[allow(dead_code)]
#[cfg(test)]
fn test_audio_stream_play() {}
