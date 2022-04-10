//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

#[path = "../src/cube_puzzle.rs"]
mod cube_puzzle;
use crate::cube_puzzle::CubePuzzle;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_apply_scramble() {
    let mut cp: CubePuzzle = CubePuzzle {
        size: 0,
        cubie_size: 0,
        gap: 0,
        image: vec![],
    };

    cp.set_cube(3);
    cp.apply_algorithm("U F' U2 F R2 B' U2 L2 R2 F D2 R2 U' B2 U' R B' F' L D2 U");
}
