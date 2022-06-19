use std::f32::consts::PI;
use chrono::{NaiveTime, Timelike};
use egui::{Color32, global_dark_light_mode_switch, Layout, Pos2, Slider, Visuals};
use crate::clock::{draw_clock, draw_clock_timestamp};
use crate::time_now;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ProgressClockApp {
    use_custom_time: bool,
    custom_time: u32,
    time_entry: (String, bool),
    clock_size: f32,
}

impl Default for ProgressClockApp {
    fn default() -> Self {
        Self {
            use_custom_time: false,
            custom_time: 45296, // 12:34:56 PM
            time_entry: ("12:34:56".to_string(), true),
            clock_size: 150.0,
        }
    }
}

impl ProgressClockApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(Visuals::dark());
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ProgressClockApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                global_dark_light_mode_switch(ui);
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.use_custom_time, "Use custom time");
                ui.with_layout(Layout::right_to_left(), |ui| {
                    if ui.button("Set to current time").clicked() {
                        let time = time_now().time();
                        self.custom_time = time.num_seconds_from_midnight();
                        self.time_entry = (
                            time.format("%I:%M:%S").to_string(),
                            time.hour12().0,
                        );
                    }
                });
            });
            if ui.add(egui::Slider::new(
                &mut self.custom_time, 0..=86399)
                .show_value(false)
            ).changed() {
                let time = NaiveTime::from_num_seconds_from_midnight(self.custom_time, 0);
                self.time_entry = (
                    time.format("%I:%M:%S").to_string(),
                    time.hour12().0,
                );
            }

            ui.horizontal(|ui| {
                let parsed_time = try_parse_time(
                    &format!("{} {}", self.time_entry.0, if self.time_entry.1 { "AM" } else { "PM" }),
                );
                let text_changed = ui.add(egui::TextEdit::singleline(&mut self.time_entry.0)
                    .text_color(if parsed_time.is_some() {
                        Color32::GREEN
                    } else {
                        Color32::RED
                    })
                ).changed();
                let pm_changed = ui.checkbox(&mut self.time_entry.1, "PM").changed();
                if text_changed || pm_changed {
                    let new_parsed_time = try_parse_time(
                        &format!("{} {}", self.time_entry.0, if self.time_entry.1 { "PM" } else { "AM" }),
                    );
                    if let Some(t) = new_parsed_time
                    {
                        self.custom_time = t;
                    }
                };
            });

            ui.add(Slider::new(&mut self.clock_size, 50.0..=300.0).text("Clock size"));
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
            let time = if self.use_custom_time {
                NaiveTime::from_num_seconds_from_midnight(self.custom_time, 0)
            } else {
                time_now().time()
            };
            ui.label(format!("{}", time));
            let painter = ui.painter();
            draw_clock_timestamp(
                painter,
                ui.max_rect().center(),
                self.clock_size,
                time,
                ctx.style().visuals.strong_text_color(),
            );
            ui.ctx().request_repaint();
        });
    }
}

fn try_parse_time(time_str: &str) -> Option<u32> {
    NaiveTime::parse_from_str(&time_str, "%I:%M:%S %p")
        .or(NaiveTime::parse_from_str(&time_str, "%I:%M %p"))
        .map(|time| time.num_seconds_from_midnight())
        .ok()
}
