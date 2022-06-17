#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod clock;

use chrono::{DateTime, Local, NaiveDateTime};
pub use app::ProgressClockApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(ProgressClockApp::new(cc))))
}

#[cfg(target_arch = "wasm32")]
fn time_now() -> NaiveDateTime {
    let ts_millis = js_sys::Date::new_0().get_time();
    let ts_secs = (ts_millis as i64) / 1000;
    let ts_ns = ((ts_millis as u32) % 1000) * 1_000_000;
    NaiveDateTime::from_timestamp(ts_secs, ts_ns)
}

#[cfg(not(target_arch = "wasm32"))]
fn time_now() -> NaiveDateTime {
    Local::now().naive_local()
}
