use inputbot::*;
use rand;
use std::{sync::mpsc, thread, time::*};
mod app;

fn main() {
    let (sender, receiver) = mpsc::channel::<app::AppState>();

    thread::spawn(move || loop {
        let state = receiver.recv().unwrap();
        state.trigger.bind(move || {
            if state.mode == app::Mode::Toggle {
                while state.trigger.is_toggled() {
                    state.button.press();
                    state.button.release();
                    thread::sleep(Duration::from_millis(state.delay));
                    if state.random == true {
                        thread::sleep(Duration::from_millis(rand::random()))
                    }
                }
            }
            if state.mode == app::Mode::Trigger {
                while state.trigger.is_pressed() {
                    state.button.press();
                    state.button.release();
                    thread::sleep(Duration::from_millis(state.delay));
                    if state.random == true {
                        thread::sleep(Duration::from_millis(rand::random()))
                    }
                }
            }
        });
    });
    thread::spawn(|| {
        handle_input_events();
    });

    eframe::run_native(
        "Autoclicker-rs",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            let app = app::AutoClickerApp {
                state: app::AppState::default(),
                sender: sender,
            };

            Box::new(app)
        }),
    );
}
