#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(target_arch = "wasm32"))]
use eframe::egui::vec2;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(vec2(1800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "CFB Time Puller",
        native_options,
        Box::new(|cc| Box::new(cfb_time_puller::app::CfbTimePuller::new(cc))),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "cfb_time_puller", // hardcode it
        web_options,
        Box::new(|cc| Box::new(cfb_time_puller::app::CfbTimePuller::new(cc))),
    )
    .expect("failed to start eframe");
}
