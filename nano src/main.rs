use eframe::egui;
use enigo::{Enigo, Mouse, Settings, Button, Direction};
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
    click_button: ClickButton,
    click_type: ClickType,
    repeat_infinite: bool,
    repeat_count: u64,
    running: Arc<AtomicBool>,
}

#[derive(PartialEq, Clone, Copy)]
enum ClickButton {
    Left,
    Right,
    Middle,
}

#[derive(PartialEq, Clone, Copy)]
enum ClickType {
    Single,
    Double,
}

impl Default for AutoClickerApp {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 100,
            click_button: ClickButton::Left,
            click_type: ClickType::Single,
            repeat_infinite: true,
            repeat_count: 100,
            running: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl AutoClickerApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Custom dark theme matching your neon-blue logo
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.window_fill = egui::Color32::from_rgb(11, 16, 25);
        style.visuals.panel_fill = egui::Color32::from_rgb(16, 22, 34);
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(24, 32, 47);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 40, 60);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 160, 220);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 190, 255);
        style.visuals.selection.bg_fill = egui::Color32::from_rgb(0, 190, 255);
        cc.egui_ctx.set_style(style);

        let app = Self::default();
        let running_clone = app.running.clone();

        // Hotkey thread (F6)
        thread::spawn(move || {
            let _ = listen(move |event: Event| {
                if let EventType::KeyPress(rdev::Key::F6) = event.event_type {
                    let current = running_clone.load(Ordering::SeqCst);
                    running_clone.store(!current, Ordering::SeqCst);
                }
            });
        });

        // Click execution thread
        let running_clicker = app.running.clone();
        thread::spawn(move || {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            let mut clicks_done: u64 = 0;

            loop {
                if running_clicker.load(Ordering::SeqCst) {
                    let interval = Duration::from_secs(app.hours * 3600 + app.minutes * 60 + app.seconds)
                        + Duration::from_millis(app.milliseconds);

                    let btn = match app.click_button {
                        ClickButton::Left => Button::Left,
                        ClickButton::Right => Button::Right,
                        ClickButton::Middle => Button::Middle,
                    };

                    let _ = enigo.button(btn, Direction::Click);
                    if app.click_type == ClickType::Double {
                        thread::sleep(Duration::from_millis(15));
                        let _ = enigo.button(btn, Direction::Click);
                    }

                    clicks_done += 1;

                    if !app.repeat_infinite && clicks_done >= app.repeat_count {
                        running_clicker.store(false, Ordering::SeqCst);
                        clicks_done = 0;
                    }
                    
                    thread::sleep(interval);
                } else {
                    clicks_done = 0;
                    thread::sleep(Duration::from_millis(50));
                }
            }
        });

        app
    }

    fn load_icon() -> egui::IconData {
        let icon_bytes = include_bytes!("apex_clicker.png");
        let img = image::load_from_memory(icon_bytes)
            .expect("Failed to load icon from memory")
            .to_rgba8();
        
        let (width, height) = img.dimensions();
        let bytes = img.into_raw();

        egui::IconData {
            rgba: bytes,
            width,
            height,
        }
    }
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("⚡ Apex Auto Clicker").color(egui::Color32::from_rgb(0, 210, 255)));
            });
            ui.separator();
            ui.add_space(5.0);

            // --- Click Interval ---
            ui.group(|ui| {
                ui.label(egui::RichText::new("Click Interval").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.add(egui::DragValue::new(&mut self.hours).range(0..=23));
                        ui.label("Hours");
                    });
                    ui.label(":");
                    ui.vertical(|ui| {
                        ui.add(egui::DragValue::new(&mut self.minutes).range(0..=59));
                        ui.label("Mins");
                    });
                    ui.label(":");
                    ui.vertical(|ui| {
                        ui.add(egui::DragValue::new(&mut self.seconds).range(0..=59));
                        ui.label("Secs");
                    });
                    ui.label(":");
                    ui.vertical(|ui| {
                        ui.add(egui::DragValue::new(&mut self.milliseconds).range(0..=999).speed(10));
                        ui.label("Millisecs");
                    });
                });
            });

            ui.add_space(8.0);

            // --- Click Settings ---
            ui.columns(2, |columns| {
                columns[0].group(|ui| {
                    ui.label(egui::RichText::new("Mouse Button").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                    ui.add_space(4.0);
                    ui.radio_value(&mut self.click_button, ClickButton::Left, "Left");
                    ui.radio_value(&mut self.click_button, ClickButton::Right, "Right");
                    ui.radio_value(&mut self.click_button, ClickButton::Middle, "Middle");
                });

                columns[1].group(|ui| {
                    ui.label(egui::RichText::new("Click Type").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                    ui.add_space(4.0);
                    ui.radio_value(&mut self.click_type, ClickType::Single, "Single");
                    ui.radio_value(&mut self.click_type, ClickType::Double, "Double");
                });
            });

            ui.add_space(8.0);

            // --- Click Repeat ---
            ui.group(|ui| {
                ui.label(egui::RichText::new("Click Repeat").strong().color(egui::Color32::from_rgb(100, 200, 255)));
                ui.add_space(4.0);
                ui.radio_value(&mut self.repeat_infinite, true, "Repeat until stopped");
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.repeat_infinite, false, "Repeat");
                    let count_enabled = !self.repeat_infinite;
                    ui.add_enabled_ui(count_enabled, |ui| {
                        ui.add(egui::DragValue::new(&mut self.repeat_count).range(1..=1000000));
                        ui.label("times");
                    });
                });
            });

            ui.add_space(12.0);

            // --- Control Panel ---
            let is_running = self.running.load(Ordering::SeqCst);
            let btn_text = if is_running { "STOP (F6)" } else { "START (F6)" };
            let btn_color = if is_running { 
                egui::Color32::from_rgb(220, 60, 60) 
            } else { 
                egui::Color32::from_rgb(0, 190, 130) 
            };
            
            ui.horizontal(|ui| {
                let button_response = ui.add_sized(
                    [160.0, 45.0],
                    egui::Button::new(
                        egui::RichText::new(btn_text)
                            .size(16.0)
                            .strong()
                            .color(egui::Color32::WHITE)
                    ).fill(btn_color)
                );
                
                if button_response.clicked() {
                    self.running.store(!is_running, Ordering::SeqCst);
                }

                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.add_space(4.0);
                    let status_text = if is_running { "Status: RUNNING" } else { "Status: IDLE" };
                    let status_color = if is_running { egui::Color32::from_rgb(0, 255, 180) } else { egui::Color32::GRAY };
                    ui.label(egui::RichText::new(status_text).strong().color(status_color));
                    ui.label(egui::RichText::new("Hotkey: F6 Toggle").size(11.0).color(egui::Color32::from_rgb(140, 160, 190)));
                });
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let app_icon = AutoClickerApp::load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([380.0, 420.0])
            .with_resizable(false)
            .with_icon(app_icon)
            .with_app_id("apex_clicker"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Apex Auto Clicker",
        options,
        Box::new(|cc| Ok(Box::new(AutoClickerApp::new(cc)))),
    )
}
