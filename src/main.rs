use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::sync::mpsc;
use std::thread;
use std::{thread::sleep, time::Duration};

//#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
//#![warn(clippy::all, rust_2018_idioms)]
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {
    let (tx, rx) = mpsc::channel::<u64>();

    thread::spawn(move || {
        F9Key.bind(|| {
            while F9Key.is_pressed() {
                LeftButton.press();
                LeftButton.release();
                sleep(Duration::from_millis(50));
            }
        });
        handle_input_events();
    });

    eframe::run_native(
        "Autoclicker-rs",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            let (sender, receiver) = mpsc::channel::<i32>();

            let app = autoclicker_rs::AutoClickerApp {
                state: autoclicker_rs::AppState::default(),
                sender: tx,
            };

            Box::new(app)
        }),
    );


}
