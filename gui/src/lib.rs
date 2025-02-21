use crossbeam_channel::Receiver;
use eframe::egui;
use status::WakeStatus;

pub mod status;

pub struct WakeUI {
    status: WakeStatus,
    rx: Receiver<WakeStatus>,
}

impl WakeUI {
    pub fn new(rx: Receiver<WakeStatus>) -> Self {
        Self {
            status: WakeStatus::Idle,
            rx,
        }
    }
}

impl eframe::App for WakeUI {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // 异步更新状态
        while let Ok(status) = self.rx.try_recv() {
            self.status = status;
        }

        egui::CentralPanel::default().show(ctx, |ui| match self.status {
            WakeStatus::Idle => {
                ui.label("Idle");
            }
            WakeStatus::Active => {
                ui.label("Waking");
            }
        });
    }
}
