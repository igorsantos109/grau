//! Grau - Motorcycle Wheelie Game
//! Rewritten in Rust for Web (WASM)

mod game;
mod physics;
mod renderer;
mod input;
mod audio;
mod bike;
mod camera;
mod ui;

use wasm_bindgen::prelude::*;
use web_sys::console;

fn log(msg: &str) {
    console::log_1(&format!("[GRAU] {}", msg).into());
}

#[wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();
    log("main() called from JS");
    
    log("Creating game...");
    let game = game::Game::new();
    log("Game created, starting loop...");
    game.run();
    log("Game loop started!");
}
