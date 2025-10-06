use js_sys::JsString;
use screeps::game;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(js_name = "loop")]
pub fn game_loop() {
    let start = game::cpu::get_used();

    game::rooms().values().for_each(|room| {
        console::log_1(&JsString::from(format!("room: {}", room.name())));
    });

    let end = game::cpu::get_used();
    console::log_1(&JsString::from(format!("cpu: {}", end - start)));
}
