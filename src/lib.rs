extern crate core;

pub mod puzzles;
mod utils;

use wasm_bindgen::prelude::*;

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

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_scramble_svg(event: &str, scramble: &str) -> String {
    set_panic_hook();

    let puzzle = puzzles::new(event);

    if let Some(mut p) = puzzle {
        p.apply_scramble(scramble);

        p.draw().to_string()
    } else {
        log("Event not recognised.");
        "".to_string()
    }
}
