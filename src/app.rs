use eframe;
use serde::{Deserialize, Serialize};
use std::sync::mpsc;

//#[derive(Serialize, Deserialize)]//#[cfg_attr(feature = "persistence", serde(tag = "type"))] // if we add new fields, give them default values when deserializing old state
#[derive(PartialEq)]
enum RadioButtonEnum {
    Left,
    Middle,
    Right,
}

//#[derive(Serialize, Deserialize)]//#[cfg_attr(feature = "persistence", serde(tag = "type"))] // if we add new fields, give them default values when deserializing old state
#[derive(PartialEq)]
enum Mode {
    Toggle,
    Trigger,
}

pub struct AutoClickerApp {
    pub state: AppState,
    pub sender: mpsc::Sender<u64>,
}

//#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "persistence", serde(tag = "type"))] // if we add new fields, give them default values when deserializing old state
pub struct AppState {
    delay: u64,
    button: RadioButtonEnum,
    mode: Mode,
    random: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            delay: 10,
            button: RadioButtonEnum::Left,
            mode: Mode::Trigger,
            random: false,
        }
    }
}

impl AutoClickerApp {
    pub fn makeapp(sender: mpsc::Sender<u64>) -> Self {
        Self {
            state: AppState::default(),
            sender,
        }
    }
}

impl eframe::App for AutoClickerApp {
    /// Called once before the first frame.
    /*fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) *//*{
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }eframe::
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }*/
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { state, sender } = self;
        let AppState {
            delay,
            button,
            mode,
            random,
        } = state;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(button, RadioButtonEnum::Left, "Left");
                    ui.radio_value(button, RadioButtonEnum::Middle, "Middle");
                    ui.radio_value(button, RadioButtonEnum::Right, "Right");
                });

                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(mode, Mode::Toggle, "Toggle");
                    ui.radio_value(mode, Mode::Trigger, "Trigger");
                });

                ui.add(egui::Slider::new(delay, 0..=10000).text("delay"));
                if ui.button("Increment").clicked() {
                    *delay += 1;
                }

                ui.add(egui::Checkbox::new(random, "Random?"));

                ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                    egui::CollapsingHeader::new("Credits and stuff").show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.hyperlink_to(
                                "Application Source Code Here",
                                "https://github.com/coding-nightmare/Autoclicker-rs",
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.label("powered by ");
                            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                            ui.label(" and ");
                            ui.hyperlink_to(
                                "eframe",
                                "https://github.com/emilk/egui/tree/master/eframe",
                            );
                        });
                        egui::warn_if_debug_build(ui);
                    });
                });
            });
        });
    }
}
