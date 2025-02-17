use crate::config::Settings;
use crate::utils::circular_buffer::CircularBuffer;
use ndarray::Array1;

struct WakeDetector {
    buffer: CircularBuffer<f32>,
    threshold: f32,
    mfcc_weight: Array1<f32>, // 预训练的唤醒词MFCC模版
}

impl WakeDetector {
    fn new(settings: &Settings) -> Self {
        Self {
            buffer: CircularBuffer::new(settings.sample_rate as usize * 2),
            threshold: settings.wake_threshold,
            mfcc_weight: Array1::zeros(13),
        }
    }

    fn process(&mut self, frame: &[f32]) -> bool {
        self.buffer.push_slice(frame);

        // 每500ms进行一次检测
        if self.buffer.len() >= self.buffer.capacity() {
            let audio = self.buffer.slices();
        }

        // 计算当前音频MFCC特征
        false
    }
}
