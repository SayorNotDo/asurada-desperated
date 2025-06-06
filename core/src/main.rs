use crossbeam_channel::unbounded;
use gui::WakeUI;
use voice::VoiceServer;
mod event;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 创建事件通道
    let (audio_sender, event_rx) = unbounded();
    let (gui_sender, gui_rx) = unbounded();

    // 启动音频服务
    let voice_server = VoiceServer::new(audio_sender).expect("failed to boot voice server");
    voice_server.audio_stream.start();

    // 启动事件循环
    tokio::spawn(event::event_loop(event_rx, gui_sender));

    // 启动 GUI
    println!("Voice Assistant Booting...");
    let _ = eframe::run_native(
        "Voice Assistant",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(WakeUI::new(gui_rx)))),
    );

    Ok(())
}
