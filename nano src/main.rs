use eframe::egui;
use enigo::{Enigo, Mouse, Settings};
use rdev::{listen, Event, EventType};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct AutoClickerApp {
    hours: u64,
    minutes: u64,
    seconds: u64,
    milliseconds: u64,
    click_type: ClickType,
    click_repeat: ClickRepeat,
    repeat_count: u64,
    running: Arc<AtomicBool>,
}

#[derive(PartialEq, Clone, Copy)]
enum ClickType {
    Left,
    Right,
    Middle,
}

#[derive(PartialEq, Clone, Copy)]
enum ClickRepeat {
    Infinite,
    Limited,
}

impl Default for AutoClickerApp {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 100,
            click_type: ClickType::Left,
            click_repeat: ClickRepeat::Infinite,
            repeat_count: 100,
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl AutoClickerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self::default();
        let running_clone = app.running.clone();

        // Background thread to handle global F6 hotkey detection via rdev
        thread::spawn(move || {
            let _ = listen(move |event: Event| {
                if let EventType::KeyPress(rdev::Key::F6) = event.event_type {
                    let current = running_clone.load(Ordering::SeqCst);
                    running_clone.store(!current, Ordering::SeqCst);
                }
            });
        });

        // Background thread to execute clicks when running is true
        let running_clicker = app.running.clone();
        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            loop {
                if running_clicker.load(Ordering::SeqCst) {
                    // Perform click based on configuration
                    // (Simplified loop placeholder for actual click execution)
                    thread::sleep(Duration::from_millis(50));
                } else {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        app
    }
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Apex Auto Clicker");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Interval:");
                ui.add(egui::DragValue::new(&mut self.hours).range(0..=23));
                ui.label("hrs");
                ui.add(egui::DragValue::new(&mut self.minutes).range(0..=59));
                ui.label("mins");
                ui.add(egui::DragValue::new(&mut self.seconds).range(0..=59));
                ui.label("secs");
                ui.add(egui::DragValue::new(&mut self.milliseconds).range(0..=999));
                ui.label("ms");
            });

            ui.add_space(10.0);

            let is_running = self.running.load(Ordering::SeqCst);
            let btn_text = if is_running { "Stop (F6)" } else { "Start (F6)" };
            if ui.button(btn_text).clicked() {
                self.running.store(!is_running, Ordering::SeqCst);
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Apex Auto Clicker",
        options,
        Box::new(|cc| Ok(Box::new(AutoClickerApp::new(cc)))),
    )
}
