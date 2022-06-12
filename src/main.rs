use inputbot::*;
use rand;
use std::{sync::mpsc, thread, time::*};
mod app;

fn main() {
    let (sender, receiver) = mpsc::channel::<app::AppState>();

    thread::spawn(move || loop {
        let state = receiver.recv().unwrap();
        let trigger = state.trigger;

        trigger.bind(move || {
            fn sleep(state: app::AppState) {
                thread::sleep(Duration::from_millis(state.delay));
                if state.random == true {
                    thread::sleep(Duration::from_millis(rand::random()))
                }
            }

            fn click(state: app::AppState) {
                state.button.press();
                state.button.release();
                sleep(state)
            }

            if state.mode == app::Mode::Toggle {
                let mut enabled = false;
                loop{
                    if trigger.is_pressed() {
                        enabled = !enabled;};
                    if trigger.is_pressed() {
                        enabled = !enabled;};
                    if trigger.is_pressed() {
                        enabled = !enabled;};
                    if trigger.is_pressed() {
                        enabled = !enabled;};
                    if trigger.is_pressed() {
                        enabled = !enabled;};
                    if enabled == true {
                        click(state);
                    }
                }
            }

            if state.mode == app::Mode::Trigger {
                while trigger.is_pressed() {
                    click(state);
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
