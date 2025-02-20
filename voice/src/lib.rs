use anyhow::Ok;
use cpal::traits::HostTrait;
use event::wake_event::WakeEvent;
use tokio::sync::mpsc::Sender;

use audio::stream::AudioStream;
use wakeword::detector::WakeDetector;

mod audio;
mod config;
pub mod event;
mod utils;
mod wakeword;

pub struct VoiceServer {
    pub audio_stream: AudioStream,
    pub detector: WakeDetector,
}

impl VoiceServer {
    pub fn new(audio_sender: Sender<WakeEvent>) -> Result<Self, anyhow::Error> {
        // 初始化配置
        let settings = config::Settings::load();

        // 启动音频采集
        let stream = audio::stream::AudioStream::new(
            &cpal::default_host().default_input_device().unwrap(),
            None,
            // Some(&settings.audio_config()),
            audio_sender,
        )?;

        // 创建唤醒词检测器
        let mut detector = WakeDetector::new(&settings);

        Ok(Self {
            audio_stream: stream,
            detector,
        })
    }
}
