/*
    MFCC（梅尔频率倒谱系数）工具函数
*/

use ndarray::{s, Array1, Array2, ArrayView1, Axis};
use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;

/// MFCC 计算器（预配置参数）
pub struct MfccExtractor {
    sample_rate: u32,         // 音频采样率
    frame_length: usize,      // 帧长（采样点数）
    frame_shift: usize,       // 帧移（采样点数）
    mel_filter_num: usize,    // 梅尔滤波器数量
    cepstrum_num: usize,      // 倒谱系数数量
    mel_filters: Array2<f32>, // 预计算的梅尔滤波器组
    dct_matrix: Array2<f32>,  // DCT变换矩阵
}

impl MfccExtractor {
    /// 创建 MFCC 计算器
    pub fn new(
        sample_rate: u32,
        frame_length: usize,
        frame_shift: usize,
        mel_filter_num: usize,
        cepstrum_num: usize,
    ) -> Self {
        // 预生成梅尔滤波器组
        let mel_filters = Self::create_mel_filter_bank(sample_rate, frame_length, mel_filter_num);

        // 预生成DCT矩阵（Type-II 离散余弦变换）
        let dct_matrix = Self::create_dct_matrix(mel_filter_num, cepstrum_num);

        Self {
            sample_rate,
            frame_length,
            frame_shift,
            mel_filter_num,
            cepstrum_num,
            mel_filters,
            dct_matrix,
        }
    }

    pub fn compute(&self, audio: &[f32]) -> Array1<f32> {
        // 预加重（高频增强）
        let pre_emphasized = self.pre_emphasize(audio);

        // 分帧 + 加窗 (Hamming)
        let frames = self.frame_and_window(pre_emphasized.view());

        // 计算功率谱
        let power_spectrum = self.fft_power_spectrum(&frames);

        // 应用梅尔滤波器组
        let mel_energies = self.apply_mel_filters(&power_spectrum);

        // 对数压缩 + DCT
        self.log_and_dct(&mel_energies)
    }

    fn pre_emphasize(&self, audio: &[f32]) -> Array1<f32> {
        let alpha = 0.97;
        let mut output = Array1::zeros(audio.len());
        output[0] = audio[0];

        for i in 1..audio.len() {
            output[i] = audio[i] - alpha * audio[i - 1];
        }

        output
    }

    fn frame_and_window(&self, audio: ArrayView1<f32>) -> Array2<f32> {
        let frame_num = 1 + (audio.len() - self.frame_length) / self.frame_shift;
        let mut frames = Array2::zeros((frame_num, self.frame_length));

        // hamming（预计算）
        let window: Array1<f32> = (0..self.frame_length)
            .map(|i| 0.54 - 0.46 * (2.0 * PI * i as f32 / (self.frame_length - 1) as f32).cos())
            .collect();

        // 分帧并加窗
        for (frame_idx, mut frame) in frames.axis_iter_mut(Axis(0)).enumerate() {
            let start = frame_idx * self.frame_shift;
            let end = start + self.frame_length;

            if end > audio.len() {
                break;
            }

            let frame_slices = audio.slice(s![start..end]);

            frame.assign(&(&frame_slices * &window));
        }

        frames
    }

    fn fft_power_spectrum(&self, frames: &Array2<f32>) -> Array2<f32> {
        let fft_size = self.frame_length;
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(fft_size);

        // 结果矩阵： 每帧的功率谱（仅保留对称部分前一半）
        let spectrum_size = fft_size / 2 + 1;
        let mut power_spectrum = Array2::zeros((frames.nrows(), spectrum_size));

        for (i, frame) in frames.axis_iter(Axis(0)).enumerate() {
            // 转换为复数输入
            let mut buffer: Vec<Complex<f32>> =
                frame.iter().map(|x| Complex::new(*x, 0.0)).collect();

            // 执行FFT
            fft.process(&mut buffer);

            // 计算功率谱（取模平方）
            for (j, bin) in buffer.iter().take(spectrum_size).enumerate() {
                power_spectrum[[i, j]] = bin.norm_sqr() / fft_size as f32;
            }
        }
        power_spectrum
    }

    /// 生成梅尔滤波器组
    fn create_mel_filter_bank(
        sample_rate: u32,
        fft_size: usize,
        mel_filter_num: usize,
    ) -> Array2<f32> {
        let nyquist = sample_rate as f32 / 2.0;
        let mel_low = 0.0;
        let mel_high = Self::hz_to_mel(nyquist);
        // 在梅尔刻度上均匀分布的点
        let mel_points = Array1::linspace(mel_low, mel_high, mel_filter_num + 2);
        let hz_points = mel_points.mapv(|m| Self::mel_to_hz(m));

        // 转换为FFT bin索引
        let bin_indices = hz_points.mapv(|hz| (hz / nyquist) * (fft_size / 2) as f32);

        // 构建滤波器组
        let mut filters = Array2::zeros((mel_filter_num, fft_size / 2 + 1));

        for i in 0..mel_filter_num {
            let left = bin_indices[i] as usize;
            let center = bin_indices[i + 1] as usize;
            let right = bin_indices[i + 2] as usize;

            // 上升坡度
            for asc in left..center {
                filters[[i, asc]] =
                    (asc as f32 - bin_indices[i]) / (bin_indices[i + 1] - bin_indices[i]);
            }

            // 下降坡度
            for desc in center..right {
                filters[[i, desc]] =
                    (bin_indices[i + 2] - desc as f32) / (bin_indices[i + 2] - bin_indices[i + 1]);
            }
        }

        filters
    }

    /// 应用梅尔滤波器组 （矩阵乘法）
    fn apply_mel_filters(&self, power_spectrum: &Array2<f32>) -> Array2<f32> {
        power_spectrum.dot(&self.mel_filters.t())
    }

    /// 对数压缩 + DCT 得到倒谱系数
    fn log_and_dct(&self, mel_energies: &Array2<f32>) -> Array1<f32> {
        // 对数能量（加1避免log(0)）
        let log_energies = mel_energies.mapv(|x| (x + 1.0).log10());

        // DCT-II 变换 (取前n_cepstrum系数)
        let cepstrum = log_energies.dot(&self.dct_matrix);
        cepstrum.row(0).to_owned() // 假设单帧输入
    }

    /// 生成DCT矩阵（Type-II）
    fn create_dct_matrix(mel_filter_num: usize, cepstrum_num: usize) -> Array2<f32> {
        let mut dct = Array2::zeros((mel_filter_num, cepstrum_num));
        let scale = (2.0 / mel_filter_num as f32).sqrt();

        for i in 0..cepstrum_num {
            for j in 0..mel_filter_num {
                dct[[j, i]] =
                    scale * ((PI / mel_filter_num as f32) * (i as f32 + 0.5) * j as f32).cos()
            }
        }

        dct
    }

    /// 频率转梅尔刻度
    fn hz_to_mel(hz: f32) -> f32 {
        2595.0 * (1.0 + hz / 700.0).log10()
    }

    /// 梅尔刻度转频率
    fn mel_to_hz(mel: f32) -> f32 {
        700.0 * (10.0f32.powf(mel / 2595.0) - 1.0)
    }
}
