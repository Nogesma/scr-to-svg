extern crate core;

mod color;
pub mod cube_puzzle;
mod dimension;
mod element;
mod rotations;
mod svg;
mod utils;

use crate::cube_puzzle::{get_cube_size, CubePuzzle};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn get_scramble_svg(event: &str, scramble: &str) -> String {
    utils::set_panic_hook();

    let cube_size = get_cube_size(event);
    if cube_size == 0 {
        log("Error: event not recognised.");
        return "".to_string();
    }

    let mut cube_puzzle = CubePuzzle::new(cube_size);

    cube_puzzle.apply_algorithm(scramble);

    cube_puzzle.draw().to_string()
}
