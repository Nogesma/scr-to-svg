use crate::puzzles::cube::{
    Cube, FiveByFive, FourByFour, SevenBySeven, SixBySix, ThreeByThree, TwoByTwo,
};
use crate::puzzles::megaminx::Megaminx;
use crate::utils::color::Color;
use std::collections::HashMap;
use svg::node::element::SVG;

mod cube;
mod megaminx;

pub trait Puzzle {
    fn new() -> Self
    where
        Self: Sized;

    fn apply_scramble(&mut self, scramble: &str);
    fn get_default_color_scheme(&self) -> ColorSchemes;

    fn draw(&self) -> SVG;
}

pub enum ColorSchemes {
    Cube(HashMap<cube::Face, Color>),
    Megaminx(HashMap<megaminx::Face, Color>),
}

pub fn new(event: &str) -> Option<Box<dyn Puzzle>> {
    match event {
        "333" | "OH" | "3BLD" => Some(Box::new(Cube::<ThreeByThree>::new())),
        "222" => Some(Box::new(Cube::<TwoByTwo>::new())),
        "444" => Some(Box::new(Cube::<FourByFour>::new())),
        "555" => Some(Box::new(Cube::<FiveByFive>::new())),
        "666" => Some(Box::new(Cube::<SixBySix>::new())),
        "777" => Some(Box::new(Cube::<SevenBySeven>::new())),
        "MEGA" => Some(Box::new(Megaminx::new())),
        _ => None,
    }
}
