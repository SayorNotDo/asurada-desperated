use crate::config::Settings;
use crate::utils::circular_buffer::CircularBuffer;
use crate::utils::mfcc::MfccExtractor;
use ndarray::Array1;

pub struct WakeDetector {
    buffer: CircularBuffer<f32>,
    threshold: f32,
    mfcc_weight: Array1<f32>, // 预训练的唤醒词MFCC模版
    mfcc_extractor: MfccExtractor,
}

impl WakeDetector {
    pub fn new(settings: &Settings) -> Self {
        let mfcc_extractor = MfccExtractor::new(settings.sample_rate, 512, 256, 26, 13);
        Self {
            buffer: CircularBuffer::new(settings.sample_rate as usize * 2),
            threshold: settings.wake_threshold,
            mfcc_weight: Array1::zeros(13),
            mfcc_extractor,
        }
    }

    pub fn process(&mut self, frame: &[f32]) -> bool {
        self.buffer.push_slice(frame);

        // 每500ms进行一次检测
        if self.buffer.len() >= self.buffer.capacity() {
            let (first_slice, second_slice) = self.buffer.slices();
            let audio = [first_slice, second_slice].concat();
            let mfcc = self.mfcc_extractor.compute(&audio); // 计算当前音频MFCC特征

            let similarity = cosine_similarity(&self.mfcc_weight, &mfcc);

            if similarity > self.threshold {
                self.buffer.clear(); // 清空缓存避免重复触发
                return true;
            }
        }
        false
    }
}

// 余弦相似度计算
fn cosine_similarity(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let dot_product = a.dot(b);
    let norm_a = a.dot(a).sqrt();
    let norm_b = b.dot(b).sqrt();

    dot_product / (norm_a * norm_b)
}
