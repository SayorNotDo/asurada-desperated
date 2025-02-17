use anyhow::Ok;
use cpal::traits::HostTrait;
use std::sync::mpsc;

use audio::stream::AudioStream;

mod audio;
mod config;
mod utils;
mod wakeword;

pub struct VoiceServer {
    audio_stream: AudioStream,
}

impl VoiceServer {
    fn new() -> Result<Self, anyhow::Error> {
        // 初始化配置
        let settings = config::Settings::load();

        // 创建事件通道
        let (audio_sender, event_rx) = mpsc::channel();

        // 启动音频采集
        let stream = audio::stream::AudioStream::new(
            &cpal::default_host().default_input_device().unwrap(),
            &settings.audio_config(),
            audio_sender,
        )?;

        Ok(Self {
            audio_stream: stream,
        })
    }
}
