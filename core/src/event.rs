use gui::status::WakeStatus;
use tokio::sync::mpsc;
use voice::event::wake_event::WakeEvent;

pub async fn event_loop(mut rx: mpsc::Receiver<WakeEvent>, gui_sender: mpsc::Sender<WakeStatus>) {
    while let Some(event) = rx.recv().await {
        match event {
            WakeEvent::AudioFrame(data) => {
                // 处理音频帧
                println!("Received audio frame: {:?}", data);
            }
            WakeEvent::WakeDetected => {
                // 唤醒词检测
                println!("Wake word detected!");
            }
        }
    }
}
