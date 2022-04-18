mod color;
pub mod cube_puzzle;
mod dimension;
mod element;
mod svg;
mod utils;

use crate::cube_puzzle::CubePuzzle;
use crate::utils::set_panic_hook;
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
    set_panic_hook();
    let mut cp: CubePuzzle = CubePuzzle {
        size: 0,
        cubie_size: 0,
        gap: 0,
        image: vec![],
    };

    cp.set_cube(event);

    if cp.size == 0 {
        log(&("Error: event not recognised.").to_string());
        return "".to_string();
    }

    return cp
        .draw_scramble(scramble, CubePuzzle::default_color_scheme())
        .to_string();
}
