mod settings;

pub struct Settings {
    pub sample_rate: u32,
    pub wake_threshold: f32,
}

impl Settings {
    pub fn load() -> Self {
        Self { sample_rate: 0 }
    }

    pub fn audio_config(&self) -> cpal::StreamConfig {
        cpal::StreamConfig {
            channels: (),
            sample_rate: cpal::SampleRate(self.sample_rate),
            buffer_size: (),
        }
    }
}
