#[derive(Debug, Clone)]
pub enum WakeEvent {
    AudioFrame(Vec<f32>),
    WakeDetected,
}

// pub async fn event_loop(mut rx: mpsc::Receiver<WakeEvent>, mut detector: WakeDetector) {
//     while let Some(event) = rx.recv().await {
//         match event {
//             WakeEvent::AudioFrame(data) => {
//                 // 处理音频帧
//                 detector.process(&data);
//                 println!("Received audio frame: {:?}", data);
//             }
//             WakeEvent::WakeDetected => {
//                 // 唤醒词检测
//                 println!("Wake word detected!");
//             }
//         }
//     }
// }
