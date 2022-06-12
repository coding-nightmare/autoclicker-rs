use eframe;
use inputbot::{KeybdKey, MouseButton};
//use serde::{Deserialize, Serialize};
use std::sync::mpsc;

//#[derive(Serialize, Deserialize)]//#[cfg_attr(feature = "persistence", serde(tag = "type"))] // if we add new fields, give them default values when deserializing old state
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Mode {
    Toggle,
    Trigger,
}

#[derive(PartialEq, Clone, Copy, Debug)]

pub enum  Trigger {
    KeybdKey(KeybdKey),
    MouseButton(MouseButton),
}

impl Trigger {
    pub fn bind<F: Fn() + Send + Sync + 'static>(self, callback: F) {
        match self {
            Trigger::KeybdKey(key) => key.bind(callback),
            Trigger::MouseButton(button) => button.bind(callback),
        }
    }
    
    pub fn is_pressed(self) -> bool {
        match self {
            Trigger::KeybdKey(key) => return key.is_pressed(),
            Trigger::MouseButton(button) => return button.is_pressed(),
        }
    }
}


pub struct AutoClickerApp {
    pub state: AppState,
    pub sender: mpsc::Sender<AppState>,
}

//#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "persistence", serde(tag = "type"))] // if we add new fields, give them default values when deserializing old state
#[derive(Clone, Copy, Debug)]
pub struct AppState {
    pub delay: u64,
    pub button: MouseButton,
    pub trigger: Trigger,
    pub mode: Mode,
    pub random: bool,
    pub clicks: u64,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            delay: 100,
            button: MouseButton::LeftButton,
            trigger: Trigger::KeybdKey(KeybdKey::F10Key),
            mode: Mode::Toggle,
            random: false,
            clicks: 100,
        }
    }
}

/*impl AutoClickerApp {
    pub fn makeapp(sender: mpsc::Sender<AppState>) -> Self {
        Self {
            state: AppState::default(),
            sender,
        }
    }
}*/

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
        #[allow(unused_variables)]
        let Self { state, sender } = self;
        let AppState {
            delay,
            button,
            mode,
            trigger,
            random,
            clicks,
        } = state;

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(button, MouseButton::LeftButton, "Left");
                    ui.radio_value(button, MouseButton::MiddleButton, "Middle");
                    ui.radio_value(button, MouseButton::RightButton, "Right");
                });

                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(mode, Mode::Trigger, "Trigger");
                    ui.radio_value(mode, Mode::Toggle, "Toggle");
                    egui::CollapsingHeader::new("").show(ui, |ui| {
                        ui.add(egui::Slider::new(clicks, 0..=1000000).text("Total Times to click"));
                    });
                });

                ui.horizontal_wrapped(|ui| {
                    //if ui.button("Select Button").clicked()  {
                    //    KeybdKey::bind_all(|event| {
                    //        println!("{:?}", event);
                    //        *trigger = event;
                    //    });
                    //};
                    egui::CollapsingHeader::new("Keybinds").show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F12Key), "F12");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F11Key), "F11");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F10Key), "F10");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F9Key), "F9");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F8Key), "F8");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F7Key), "F7");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F6Key), "F6");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F5Key), "F5");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F4Key), "F4");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F3Key), "F3");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F2Key), "F2");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::F1Key), "F1");
                            ui.radio_value(trigger, Trigger::KeybdKey(KeybdKey::NumLockKey), "Numlock");
                            ui.radio_value(trigger, Trigger::MouseButton(MouseButton::LeftButton), "Left");
                            ui.radio_value(trigger, Trigger::MouseButton(MouseButton::RightButton), "Right");
                            ui.radio_value(trigger, Trigger::MouseButton(MouseButton::MiddleButton), "Middle");
                            ui.radio_value(trigger, Trigger::MouseButton(MouseButton::X1Button), "MouseX1");
                            ui.radio_value(trigger, Trigger::MouseButton(MouseButton::X2Button), "MouseX2");
                        });
                    });
                });

                ui.add(egui::Slider::new(delay, 0..=1000).text("delay"));
                if ui.button("Increment").clicked() {
                    *delay += 10;
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

        sender.send(*state).unwrap();
    }
}
