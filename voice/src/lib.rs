use anyhow::Ok;
use cpal::traits::HostTrait;
use crossbeam_channel::Sender;
use event::wake_event::WakeEvent;

use audio::stream::AudioStream;

mod audio;
mod config;
pub mod event;
mod utils;
pub mod wakeword;

pub struct VoiceServer {
    pub audio_stream: AudioStream,
}

impl VoiceServer {
    pub fn new(audio_sender: Sender<WakeEvent>) -> Result<Self, anyhow::Error> {
        // 初始化配置
        // let settings = config::Settings::load();

        // 启动音频采集
        let stream = audio::stream::AudioStream::new(
            &cpal::default_host().default_input_device().unwrap(),
            None,
            // Some(&settings.audio_config()),
            audio_sender,
        )?;

        Ok(Self {
            audio_stream: stream,
        })
    }
}
