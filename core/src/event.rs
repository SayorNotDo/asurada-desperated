use crossbeam_channel::{Receiver, Sender};
use gui::status::WakeStatus;
use voice::event::wake_event::WakeEvent;
use voice::wakeword::detector::WakeDetector;

pub async fn event_loop(mut rx: Receiver<WakeEvent>, gui_sender: Sender<WakeStatus>) {
    let mut detector = WakeDetector::new();
    loop {
        // 同步阻塞接受（非异步）
        match rx.recv() {
            Ok(event) => {
                match event {
                    WakeEvent::AudioFrame(data) => {
                        // 处理音频帧
                        if detector.process(&data) {
                            println!("Received audio frame");
                            // 触发控制指令（车控、UI）
                        }
                    }
                    WakeEvent::WakeDetected => {
                        // 处于已唤醒状态
                        println!("Wake word detected!");
                    }
                }
            }
            Err(crossbeam_channel::RecvError) => {
                // 通道关闭时退出循环
                println!("Audio channel closed, existing event loop");
                break;
            }
        }
    }
}
