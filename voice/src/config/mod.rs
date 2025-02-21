pub struct Settings {
    pub channels: u16,
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub wake_threshold: f32,
    pub wakeword_path: String,
}

impl Settings {
    pub fn load() -> Self {
        Self {
            channels: 1,
            sample_rate: 96000,
            wake_threshold: 0.0,
            buffer_size: 0,
            wakeword_path: "".into(),
        }
    }

    pub fn audio_config(&self) -> cpal::StreamConfig {
        cpal::StreamConfig {
            channels: self.channels,
            sample_rate: cpal::SampleRate(self.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(self.buffer_size),
        }
    }
}
