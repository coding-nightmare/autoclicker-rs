use inputbot::{KeybdKey::*, MouseButton::*, *};
use std::sync::mpsc;
use std::thread;
use std::{thread::sleep, time::Duration};
mod app;

//#![forbid(unsafe_code)]
//#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
//#![warn(clippy::all, rust_2018_idioms)]
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]

fn main() {
    let (sender, receiver) = mpsc::channel::<app::AppState>();

    thread::spawn(move || {
    });

    eframe::run_native(
        "Autoclicker-rs",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            
            let app = app::AutoClickerApp {
                state: app::AppState::default(),
                sender: sender,
            };

            Box::new(app)
        }),
    );
}
