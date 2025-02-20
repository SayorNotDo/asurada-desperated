use crossbeam_channel::Receiver;
use gui::status::WakeStatus;
use voice::event::wake_event::WakeEvent;

pub async fn event_loop(mut rx: Receiver<WakeEvent>, gui_sender: mpsc::Sender<WakeStatus>) {
    while let Some(event) = rx.recv() {
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
