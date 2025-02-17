use std::sync::mpsc;

pub enum WakeEvent {
    AudioFrame(Vec<f32>),
    WakeDetected,
}

pub async fn event_loop(mut rx: mpsc::Receiver<WakeEvent>) {}
