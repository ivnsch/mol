use log::{info, Level};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_log() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(Level::Info));
    info!("Initialized wasm logs");
    Ok(())
}
