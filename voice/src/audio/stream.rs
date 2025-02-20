use crate::event::wake_event::WakeEvent;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    Stream,
};
use std::time;
use tokio::sync::mpsc::Sender;

pub struct AudioStream {
    pub stream: Stream,
}

impl AudioStream {
    pub fn new(
        device: &cpal::Device,
        config: Option<&cpal::StreamConfig>,
        event_sender: Sender<WakeEvent>,
    ) -> Result<Self, anyhow::Error> {
        list_supported_configs(&device);
        // 创建音频输入流
        let stream_config = match config {
            Some(config) => config.clone(),
            None => get_compatible_config(device)?,
        };

        let stream = device.build_input_stream(
            &stream_config,
            move |data: &[f32], _| {
                // 发送原始音频数据到唤醒词检查模块
                let sender = event_sender.clone();
                let data_copy = data.to_vec();
                // 不在tokio运行时内，需要手动创建一个tokio任务
                tokio::spawn(async move {
                    sender
                        .send(WakeEvent::AudioFrame(data_copy))
                        .await
                        .expect("failed to send audio frame");
                });
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

fn list_supported_configs(device: &cpal::Device) {
    println!("supported configuration: ");
    let configs = device.supported_input_configs().unwrap();
    for config in configs {
        println!("{:?}", config);
    }
}

fn get_compatible_config(device: &cpal::Device) -> Result<cpal::StreamConfig, anyhow::Error> {
    let mut configs = device.supported_input_configs()?;

    // 优先选择 f32 格式、16kHz、单声道的音频流
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
