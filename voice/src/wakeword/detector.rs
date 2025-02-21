use std::fs::File;
use std::io::{Cursor, Read};

use crate::config::Settings;
use crate::utils::circular_buffer::CircularBuffer;
use crate::utils::mfcc::MfccExtractor;
use anyhow::{Context, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use ndarray::Array1;

pub struct WakeDetector {
    buffer: CircularBuffer<f32>,
    threshold: f32,
    mfcc_weight: Array1<f32>, // 预训练的唤醒词MFCC模版
    mfcc_extractor: MfccExtractor,
}

impl WakeDetector {
    pub fn new() -> Self {
        let settings = Settings::load();
        let mfcc_extractor = MfccExtractor::new(settings.sample_rate, 512, 256, 26, 13);
        let mfcc_weight = load_mfcc_template(&settings.wakeword_path).unwrap();
        Self {
            buffer: CircularBuffer::new(settings.sample_rate as usize * 2),
            threshold: settings.wake_threshold,
            mfcc_weight,
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
            println!("current mfcc: {:?}", mfcc);
            let similarity = cosine_similarity(&self.mfcc_weight, &mfcc);
            println!("similariity indicator: {}", similarity);
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
    println!("factor a: {:?}, factor b: {:?}", a, b);
    let dot_product = a.dot(b);
    let norm_a = a.dot(a).sqrt();
    let norm_b = b.dot(b).sqrt();

    dot_product / (norm_a * norm_b)
}

/// 从二进制文件加载预训练的MFCC模版
fn load_mfcc_template(path: &str) -> Result<Array1<f32>, anyhow::Error> {
    let mut file =
        File::open(path).with_context(|| format!("failed to open MFCC template file: {}", path))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .with_context(|| "failed to read binary file")?;

    // 每4字节转换为一个f32（控制为小端序）
    let mfcc_data: Vec<f32> = buffer
        .chunks_exact(4)
        .map(|chunk| {
            let mut reader = Cursor::new(chunk);
            reader
                .read_f32::<LittleEndian>()
                .with_context(|| "failed to parse")
        })
        .collect::<Result<_>>()?;
    Ok(Array1::from_vec(mfcc_data))
}
