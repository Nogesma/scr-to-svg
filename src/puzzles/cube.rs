use crate::puzzles::{ColorSchemes, Puzzle};
use crate::utils::color::Color;
use crate::utils::maths::rotate_2d_matrix;
use lazy_static::lazy_static;
use ndarray::{s, Array1, Array3};
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use svg::node::element::{Group, Rectangle, SVG};
use svg::{Document, Node};

#[derive(Debug)]
pub struct Cube<T> {
    pub state: Array3<Face>,
    size: usize,
    color_scheme: HashMap<Face, Color>,
    cubie_size: usize,
    gap: usize,
    _marker: PhantomData<T>,
}

#[derive(Debug, Default)]
pub struct TwoByTwo {}
#[derive(Debug, Default)]
pub struct ThreeByThree {}
#[derive(Debug, Default)]
pub struct FourByFour {}
#[derive(Debug, Default)]
pub struct FiveByFive {}
#[derive(Debug, Default)]
pub struct SixBySix {}
#[derive(Debug, Default)]
pub struct SevenBySeven {}

#[derive(Debug)]
struct Move {
    face: Face,
    dir: Direction,
    depth: usize,
    wide: bool,
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    None,
    Clockwise,
    Half,
    Counterclockwise,
}

impl From<usize> for Direction {
    fn from(x: usize) -> Self {
        match x {
            0 => Direction::None,
            1 => Direction::Clockwise,
            2 => Direction::Half,
            3 => Direction::Counterclockwise,
            _ => panic!(),
        }
    }
}

impl From<TwoByTwo> for usize {
    fn from(_: TwoByTwo) -> Self {
        2
    }
}
impl From<ThreeByThree> for usize {
    fn from(_: ThreeByThree) -> Self {
        3
    }
}
impl From<FourByFour> for usize {
    fn from(_: FourByFour) -> Self {
        4
    }
}
impl From<FiveByFive> for usize {
    fn from(_: FiveByFive) -> Self {
        5
    }
}
impl From<SixBySix> for usize {
    fn from(_: SixBySix) -> Self {
        6
    }
}
impl From<SevenBySeven> for usize {
    fn from(_: SevenBySeven) -> Self {
        7
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Face {
    R,
    U,
    F,
    L,
    D,
    B,
}

impl From<usize> for Face {
    fn from(x: usize) -> Self {
        match x {
            0 => Face::R,
            1 => Face::U,
            2 => Face::F,
            3 => Face::L,
            4 => Face::D,
            5 => Face::B,
            _ => panic!(),
        }
    }
}

impl From<char> for Face {
    fn from(x: char) -> Self {
        match x.to_ascii_uppercase() {
            'R' => Face::R,
            'U' => Face::U,
            'F' => Face::F,
            'L' => Face::L,
            'D' => Face::D,
            'B' => Face::B,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl PartialEq<usize> for Face {
    fn eq(&self, other: &usize) -> bool {
        self == &Face::from(*other)
    }
}

impl PartialEq<Face> for usize {
    fn eq(&self, other: &Face) -> bool {
        &Face::from(*self) == other
    }
}

lazy_static! {
    pub static ref DEFAULT_COLOR_SCHEME: HashMap<Face, Color> = HashMap::from([
        (Face::B, Color::blue()),
        (Face::D, Color::yellow()),
        (Face::F, Color::green()),
        (Face::L, Color::orange()),
        (Face::R, Color::red()),
        (Face::U, Color::white()),
    ]);
}

impl<T> Puzzle for Cube<T>
where
    usize: From<T>,
    T: Default,
{
    fn new() -> Self {
        let size = T::into(T::default());

        Cube {
            size,
            cubie_size: 10,
            gap: 2,
            state: Array3::<Face>::from_shape_fn((6, size, size), |(i, _, _)| Face::from(i)),
            color_scheme: DEFAULT_COLOR_SCHEME.clone(),
            _marker: Default::default(),
        }
    }

    fn apply_scramble(&mut self, scramble: &str) {
        let moves = scramble
            .split_ascii_whitespace()
            .map(Cube::<T>::parse_moves);
        moves.for_each(|mut mv| self.apply_move(&mut mv));
    }

    fn get_default_color_scheme(&self) -> ColorSchemes {
        ColorSchemes::Cube(DEFAULT_COLOR_SCHEME.clone())
    }

    fn draw(&self) -> SVG {
        let (width, height) = self.get_preferred_size();

        let mut svg = Document::new()
            .set("viewBox", (0, 0, width, height))
            .set("height", "100%")
            .set("width", "100%");

        self.draw_cube(&mut svg);
        svg
    }
}

impl<T> Cube<T> {
    fn get_dir(mut mv: String, dir: bool) -> Direction {
        if dir {
            mv.pop();
        }
        let x = mv.parse::<i8>().unwrap_or(1) % 4;
        Direction::from(if dir { (-x).rem_euclid(4) } else { x } as usize)
    }

    fn parse_moves(mv: &str) -> Move {
        let a = mv.find(char::is_alphabetic).unwrap();

        let pre: String = mv.chars().take(a).collect();
        let suf: String = mv
            .chars()
            .rev()
            .take_while(|x| !x.is_alphabetic())
            .collect();
        let mv: String = mv
            .chars()
            .skip(a)
            .take_while(|x| x.is_alphabetic())
            .collect();

        let wide = mv.contains('w');
        let default = if wide { 2 } else { 1 };
        let depth = pre.parse().unwrap_or(default) - 1;
        let dir = Cube::<T>::get_dir(suf.clone(), suf.ends_with('\''));

        let f = mv.chars().next().unwrap();

        if wide && f.is_lowercase() {
            panic!("Move cannot be wide and slice at the same time");
        }

        let face = Face::from(f);
        Move {
            face,
            depth,
            dir,
            wide,
        }
    }

    fn draw_cube(&self, svg: &mut SVG) {
        let gap = self.gap;
        let size = self.size;
        let cubie_size = self.cubie_size;

        let mut g = Group::new().set("transform", "translate(0.5,0.5)");

        self.paint_cube_face(
            &mut g,
            3 * gap + 2 * size * cubie_size,
            2 * gap + size * cubie_size,
            Face::R,
        );

        self.paint_cube_face(&mut g, 2 * gap + size * cubie_size, gap, Face::U);

        self.paint_cube_face(
            &mut g,
            2 * gap + size * cubie_size,
            2 * gap + size * cubie_size,
            Face::F,
        );

        self.paint_cube_face(&mut g, gap, 2 * gap + size * cubie_size, Face::L);

        self.paint_cube_face(
            &mut g,
            2 * gap + size * cubie_size,
            3 * gap + 2 * size * cubie_size,
            Face::D,
        );

        self.paint_cube_face(
            &mut g,
            4 * gap + 3 * size * cubie_size,
            2 * gap + size * cubie_size,
            Face::B,
        );

        svg.append(g);
    }

    fn paint_cube_face(&self, g: &mut Group, x: usize, y: usize, face: Face) {
        let size = self.size;
        let cubie_size = self.cubie_size;

        for row in 0..size {
            for col in 0..size {
                let x = x + col * cubie_size;
                let y = y + row * cubie_size;

                let color = if let Some(col) = self
                    .color_scheme
                    .get(&self.state.slice(s![face as usize, .., ..])[[row, col]])
                {
                    *col
                } else {
                    Color::black()
                };

                let rect = Rectangle::new()
                    .set("x", x)
                    .set("y", y)
                    .set("width", cubie_size)
                    .set("height", cubie_size)
                    .set("fill", color.to_string())
                    .set("stroke", Color::black().to_string());

                g.append(rect);
            }
        }
    }

    fn get_preferred_size(&self) -> (usize, usize) {
        let width = (self.size * self.cubie_size + self.gap) * 4 + self.gap;
        let height = (self.size * self.cubie_size + self.gap) * 3 + self.gap;

        (width, height)
    }

    fn get_layer_idx(&self, i: usize, cond: bool) -> [usize; 2] {
        if cond {
            [self.size - i - 1, i]
        } else {
            [i, self.size - i - 1]
        }
    }

    fn ap(&mut self, mv: &Move) {
        let clockwise = mv.dir == Direction::Clockwise;
        let mut face = self.state.slice_mut(s![mv.face as usize, .., ..]);
        face.assign(&rotate_2d_matrix(&mut face.to_owned(), clockwise));

        match mv {
            Move {
                face: Face::R | Face::L,
                wide,
                depth,
                ..
            } => {
                let a = [Face::U, Face::F, Face::D, Face::B];
                let b = [Face::D, Face::F, Face::U, Face::B];

                let r = match (mv.face, clockwise) {
                    (Face::R, true) => a,
                    (Face::R, false) => b,
                    (Face::L, true) => b,
                    (Face::L, false) => a,
                    _ => panic!(),
                };

                self.rotate_rl_layer(r, self.get_layer_idx(*depth, mv.face == Face::R));
                if *wide {
                    for i in 0..*depth {
                        self.rotate_rl_layer(r, self.get_layer_idx(i, mv.face == Face::R));
                    }
                }
            }
            Move {
                face: Face::U | Face::D,
                wide,
                depth,
                ..
            } => {
                let a = [Face::R, Face::B, Face::L, Face::F];
                let b = [Face::R, Face::F, Face::L, Face::B];

                let r = match (mv.face, clockwise) {
                    (Face::U, true) => a,
                    (Face::U, false) => b,
                    (Face::D, true) => b,
                    (Face::D, false) => a,
                    _ => panic!(),
                };

                self.rotate_ud_layer(r, self.get_layer_idx(*depth, mv.face == Face::D)[0]);
                if *wide {
                    for i in 0..*depth {
                        self.rotate_ud_layer(r, self.get_layer_idx(i, mv.face == Face::D)[0]);
                    }
                }
            }
            Move {
                face: Face::F | Face::B,
                wide,
                depth,
                ..
            } => {
                let r = if clockwise {
                    [Face::U, Face::L, Face::D, Face::R]
                } else {
                    [Face::D, Face::R, Face::U, Face::L]
                };

                let mut rotate_fb = if mv.face == Face::F && !clockwise
                    || mv.face == Face::B && clockwise
                {
                    Box::new(|a, b| self.rev_rotate_fb_layer(a, b))
                        as Box<dyn FnMut([Face; 4], usize)>
                } else {
                    Box::new(|a, b| self.rotate_fb_layer(a, b)) as Box<dyn FnMut([Face; 4], usize)>
                };

                rotate_fb(r, *depth);
                if *wide {
                    for i in 0..*depth {
                        rotate_fb(r, i);
                    }
                }
            }
        }
    }

    fn apply_move(&mut self, mv: &mut Move) {
        match mv.dir {
            Direction::None => {}
            Direction::Half => {
                mv.dir = Direction::Clockwise;
                self.ap(mv);
                self.ap(mv);
            }
            Direction::Clockwise | Direction::Counterclockwise => {
                self.ap(mv);
            }
        }
    }

    fn rotate_rl_layer(&mut self, r: [Face; 4], side: [usize; 2]) {
        let tmp: Array1<Face> = self
            .state
            .slice(s![r[0] as usize, .., side[0]])
            .iter()
            .copied()
            .rev()
            .collect();

        let b = self.state.slice(s![r[1] as usize, .., side[0]]).to_owned();
        self.state
            .slice_mut(s![r[0] as usize, .., side[0]])
            .assign(&b);

        let c = self.state.slice(s![r[2] as usize, .., side[0]]).to_owned();
        self.state
            .slice_mut(s![r[1] as usize, .., side[0]])
            .assign(&c);

        let d: Array1<Face> = self
            .state
            .slice(s![r[3] as usize, .., side[1]])
            .iter()
            .copied()
            .rev()
            .collect();
        self.state
            .slice_mut(s![r[2] as usize, .., side[0]])
            .assign(&d);
        self.state
            .slice_mut(s![r[3] as usize, .., side[1]])
            .assign(&tmp);
    }

    fn rotate_ud_layer(&mut self, r: [Face; 4], side: usize) {
        let tmp = self.state.slice(s![r[0] as usize, side, ..]).to_owned();
        for i in 0..3 {
            let b = self.state.slice(s![r[i + 1] as usize, side, ..]).to_owned();
            self.state.slice_mut(s![r[i] as usize, side, ..]).assign(&b);
        }
        self.state
            .slice_mut(s![r[3] as usize, side, ..])
            .assign(&tmp);
    }

    fn rotate_fb_layer(&mut self, r: [Face; 4], layer: usize) {
        let tmp = self
            .state
            .slice(s![r[0] as usize, self.size - layer - 1, ..])
            .to_owned();

        let b: Array1<Face> = self
            .state
            .slice(s![r[1] as usize, .., self.size - layer - 1])
            .iter()
            .copied()
            .rev()
            .collect();
        self.state
            .slice_mut(s![r[0] as usize, self.size - layer - 1, ..])
            .assign(&b);

        let c = self.state.slice(s![r[2] as usize, layer, ..]).to_owned();
        self.state
            .slice_mut(s![r[1] as usize, .., self.size - layer - 1])
            .assign(&c);

        let d: Array1<Face> = self
            .state
            .slice(s![r[3] as usize, .., layer])
            .iter()
            .copied()
            .rev()
            .collect();
        self.state
            .slice_mut(s![r[2] as usize, layer, ..])
            .assign(&d);

        self.state
            .slice_mut(s![r[3] as usize, .., layer])
            .assign(&tmp);
    }

    fn rev_rotate_fb_layer(&mut self, r: [Face; 4], layer: usize) {
        let tmp = self
            .state
            .slice(s![r[2] as usize, self.size - layer - 1, ..])
            .to_owned();

        let b = self.state.slice(s![r[1] as usize, .., layer]).to_owned();
        self.state
            .slice_mut(s![r[2] as usize, self.size - layer - 1, ..])
            .assign(&b);

        for i in 0..self.size {
            self.state[[r[1] as usize, i, layer]] =
                self.state[[r[0] as usize, layer, self.size - i - 1]];
        }

        for j in 0..self.size {
            self.state[[r[0] as usize, layer, j]] =
                self.state[[r[3] as usize, j, self.size - layer - 1]];
        }

        for i in 0..self.size {
            self.state[[r[3] as usize, i, self.size - layer - 1]] = tmp[self.size - i - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::puzzles::cube::Face::{B, D, F, L, R, U};
    use ndarray::arr3;

    #[test]
    fn three_by_three() {
        {
            let mut cube = Cube::<ThreeByThree>::new();

            cube.apply_scramble("U F' U2 F R2 B' U2 L2 R2 F D2 R2 U' B2 U' R B' F' L D2 U");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[1, 5, 4], [1, 0, 4], [5, 1, 0]],
                    [[1, 0, 0], [2, 1, 3], [2, 5, 3]],
                    [[0, 0, 2], [0, 2, 3], [4, 4, 3]],
                    [[0, 1, 4], [5, 3, 2], [3, 3, 2]],
                    [[3, 3, 1], [2, 4, 0], [4, 4, 5]],
                    [[5, 4, 2], [5, 5, 1], [1, 2, 5]],
                ])
            );
        }
        {
            let mut cube = Cube::<ThreeByThree>::new();

            cube.apply_scramble("B F2 U2 B2 F2 R U2 L2 R U' B D2 L' D' L2 U2 B D' F2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[4, 2, 3], [2, 0, 0], [2, 1, 0]],
                    [[3, 4, 2], [3, 1, 3], [2, 0, 5]],
                    [[1, 5, 0], [5, 2, 1], [5, 1, 1]],
                    [[5, 5, 0], [4, 3, 1], [0, 0, 4]],
                    [[3, 0, 3], [2, 4, 3], [1, 5, 2]],
                    [[4, 2, 1], [4, 5, 3], [4, 4, 5]]
                ])
            );
        }
        {
            let mut cube = Cube::<ThreeByThree>::new();

            cube.apply_scramble("F U R U2 B U2 B' D2 B L2 D2 L2 D F U L' D' L B2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[2, 3, 1], [5, 0, 4], [3, 2, 3]],
                    [[1, 0, 2], [4, 1, 1], [3, 2, 4]],
                    [[4, 1, 0], [0, 2, 1], [0, 5, 1]],
                    [[0, 2, 5], [0, 3, 5], [4, 5, 5]],
                    [[1, 3, 5], [4, 4, 3], [0, 4, 4]],
                    [[3, 1, 2], [0, 5, 2], [2, 3, 5]]
                ])
            );
        }
        {
            let mut cube = Cube::<ThreeByThree>::new();

            cube.apply_scramble("U2 R F2 L D2 B2 F2 L2 U' L' U2 L2 D F U B' L2 U F2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[5, 5, 1], [4, 0, 2], [0, 2, 2]],
                    [[1, 1, 3], [1, 1, 3], [2, 0, 4]],
                    [[4, 1, 3], [0, 2, 0], [3, 0, 4]],
                    [[0, 3, 3], [4, 3, 2], [1, 1, 2]],
                    [[1, 5, 5], [5, 4, 4], [5, 3, 4]],
                    [[5, 2, 2], [3, 5, 5], [0, 4, 0]]
                ])
            );
        }
        {
            let mut cube = Cube::<ThreeByThree>::new();

            cube.apply_scramble("U' L2 U L' U2 D' F' B' R2 F' R B L2 U2 B D2 F2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[0, 2, 3], [3, 0, 2], [0, 0, 2]],
                    [[1, 0, 4], [1, 1, 4], [2, 0, 5]],
                    [[4, 2, 1], [5, 2, 1], [1, 4, 2]],
                    [[3, 5, 3], [1, 3, 3], [4, 4, 3]],
                    [[2, 5, 4], [0, 4, 1], [0, 4, 0]],
                    [[5, 5, 5], [3, 5, 2], [1, 3, 5]]
                ])
            );
        }
    }

    #[test]
    fn two_by_two() {
        {
            let mut cube = Cube::<TwoByTwo>::new();

            cube.apply_scramble("U' R F' R2 F R U2 R' F' U2 R2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[5, 2,], [3, 0,],],
                    [[5, 3,], [3, 0,],],
                    [[2, 4,], [2, 1,],],
                    [[0, 4,], [3, 4,],],
                    [[0, 5,], [4, 1,],],
                    [[1, 1,], [2, 5,],],
                ])
            );
        }
        {
            let mut cube = Cube::<TwoByTwo>::new();

            cube.apply_scramble("R' U2 R' F' R U' R F' R' U R'");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[5, 0], [4, 4]],
                    [[0, 5], [3, 3]],
                    [[1, 1], [0, 2]],
                    [[2, 2], [3, 5]],
                    [[1, 3], [4, 0]],
                    [[4, 1], [2, 5]]
                ])
            );
        }
        {
            let mut cube = Cube::<TwoByTwo>::new();

            cube.apply_scramble("U' R' F U' F' U2 R2 U2 R2 U' F");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[2, 0], [3, 2]],
                    [[2, 5], [1, 4]],
                    [[5, 0], [1, 2]],
                    [[4, 0], [3, 5]],
                    [[3, 1], [4, 0]],
                    [[4, 3], [1, 5]]
                ])
            );
        }
        {
            let mut cube = Cube::<TwoByTwo>::new();

            cube.apply_scramble("R U' R' U F2 U' F U2 F R U");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[2, 3], [0, 1]],
                    [[0, 5], [0, 3]],
                    [[2, 4], [2, 4]],
                    [[4, 1], [3, 1]],
                    [[3, 5], [4, 0]],
                    [[1, 2], [5, 5]]
                ])
            );
        }
        {
            let mut cube = Cube::<TwoByTwo>::new();

            cube.apply_scramble("R' U2 F' U' F R' U' F U2 F U'");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[1, 1], [4, 3]],
                    [[3, 0], [3, 0]],
                    [[1, 5], [0, 0]],
                    [[2, 2], [3, 4]],
                    [[5, 2], [4, 1]],
                    [[2, 4], [5, 5]]
                ])
            );
        }
    }

    #[test]
    fn four_by_four() {
        {
            let mut cube = Cube::<FourByFour>::new();

            cube.apply_scramble("D2 B2 R2 L B2 D2 R B2 F' L2 D2 R L2 D L' B D L' B2 Rw2 B2 U Rw2 Fw2 Uw2 L' D2 L Fw2 U R F2 B L2 Fw L2 B Uw Rw' F' Rw Fw' U2 Rw2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[3, 2, 3, 1], [5, 0, 0, 5], [4, 5, 0, 0], [3, 0, 4, 4]],
                    [[1, 3, 4, 0], [2, 1, 3, 2], [0, 4, 1, 0], [5, 1, 3, 5]],
                    [[4, 0, 2, 4], [4, 0, 2, 3], [1, 4, 1, 3], [3, 1, 5, 2]],
                    [[5, 1, 2, 0], [5, 5, 2, 2], [1, 1, 3, 5], [4, 4, 0, 1]],
                    [[5, 2, 3, 1], [4, 5, 2, 4], [3, 4, 3, 5], [3, 2, 1, 0]],
                    [[2, 5, 1, 0], [1, 4, 3, 0], [5, 2, 5, 3], [2, 0, 4, 2]]
                ])
            );
        }
        {
            let mut cube = Cube::<FourByFour>::new();

            cube.apply_scramble("D L B R' F2 R2 D2 L2 D2 R' B2 L F2 D' L' F' U' R' D' R2 Fw2 U2 Rw2 L Fw2 R2 U' R' U2 D' R Uw2 D' Fw' L2 U' Fw R2 Fw2 L' F2 Rw Uw' L2 Fw' Uw");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[3, 0, 3, 5], [2, 3, 0, 4], [5, 0, 3, 4], [0, 5, 2, 0]],
                    [[2, 1, 0, 0], [2, 2, 5, 4], [5, 1, 2, 1], [5, 0, 5, 2]],
                    [[3, 2, 0, 1], [3, 2, 3, 4], [0, 0, 5, 4], [1, 5, 1, 4]],
                    [[4, 0, 3, 4], [4, 0, 4, 2], [3, 2, 1, 4], [0, 1, 3, 5]],
                    [[3, 4, 2, 5], [5, 4, 1, 1], [5, 1, 5, 1], [2, 1, 0, 2]],
                    [[1, 1, 3, 3], [0, 4, 4, 2], [3, 3, 5, 2], [4, 5, 3, 1]]
                ])
            );
        }
        {
            let mut cube = Cube::<FourByFour>::new();

            cube.apply_scramble("F R D2 F2 U2 R2 B2 U2 L2 D L' B' U F2 R' F B2 U' L2 Fw2 R B' Uw2 Rw2 R' U2 R2 B2 D2 Rw2 Fw2 B' Uw' F' Rw2 R2 Uw2 U2 Fw' R2 Uw Fw' Rw2 L2 Uw");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[0, 2, 1, 5], [4, 2, 4, 4], [2, 2, 0, 4], [1, 5, 5, 0]],
                    [[0, 3, 2, 3], [5, 0, 5, 0], [5, 1, 4, 4], [3, 2, 1, 2]],
                    [[2, 4, 3, 4], [3, 1, 2, 3], [0, 0, 5, 0], [3, 2, 3, 3]],
                    [[2, 0, 1, 4], [0, 1, 5, 2], [1, 1, 3, 4], [1, 0, 2, 1]],
                    [[5, 1, 1, 2], [1, 0, 4, 3], [4, 4, 2, 3], [5, 5, 1, 4]],
                    [[4, 0, 2, 1], [3, 3, 3, 5], [5, 3, 5, 0], [5, 5, 4, 0]]
                ])
            );
        }
        {
            let mut cube = Cube::<FourByFour>::new();

            cube.apply_scramble("D2 F2 D F2 R2 D' F2 D2 B2 L R U F' L' U2 F2 L' B' L U2 Rw2 Fw2 U F' B U F' Rw2 D2 F' Uw2 R' U2 Rw F' L' D' B' Uw Rw' Fw U B2 R2 Uw");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[2, 3, 5, 0], [2, 1, 5, 3], [1, 3, 2, 4], [2, 2, 1, 5]],
                    [[4, 5, 5, 1], [1, 4, 3, 3], [0, 1, 4, 2], [1, 1, 5, 3]],
                    [[0, 3, 1, 4], [0, 0, 3, 0], [4, 2, 4, 0], [3, 5, 0, 1]],
                    [[0, 2, 4, 2], [3, 5, 3, 5], [1, 0, 0, 2], [2, 4, 2, 5]],
                    [[4, 4, 1, 3], [0, 2, 5, 1], [5, 1, 2, 5], [0, 3, 4, 3]],
                    [[5, 0, 3, 5], [4, 4, 5, 4], [2, 0, 1, 3], [1, 0, 2, 4]]
                ])
            );
        }
        {
            let mut cube = Cube::<FourByFour>::new();

            cube.apply_scramble("D' F U' L2 D U' L2 U R2 D F' U' R D' L2 F' L D' R' Rw2 U' Rw2 Uw2 Fw2 L' U L' D Rw2 Uw2 D Rw2 Fw U' L2 D Rw' Uw2 Fw2 L2 Fw' R U2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [[4, 3, 3, 2], [4, 4, 2, 3], [5, 1, 0, 2], [1, 4, 2, 4]],
                    [[0, 4, 5, 4], [4, 3, 0, 1], [1, 1, 2, 5], [2, 0, 1, 5]],
                    [[0, 2, 2, 0], [5, 2, 3, 0], [1, 0, 5, 1], [1, 1, 5, 3]],
                    [[2, 2, 0, 4], [3, 4, 1, 0], [0, 4, 5, 5], [1, 0, 4, 0]],
                    [[5, 3, 4, 2], [3, 1, 3, 3], [5, 4, 2, 1], [3, 3, 4, 3]],
                    [[3, 4, 0, 1], [2, 5, 0, 5], [0, 3, 5, 1], [5, 2, 2, 5]]
                ])
            );
        }
    }
    #[test]
    fn five_by_five() {
        {
            let mut cube = Cube::<FiveByFive>::new();

            cube.apply_scramble("Uw2 Dw2 Lw Rw F2 B' D' F R U Dw2 Uw' Lw' Uw' D F' B2 Bw D2 Dw U2 B F2 Bw U' Rw2 Uw U2 Dw2 B2 Lw' Bw2 Uw' Bw' Rw D2 Dw' U L2 B' D Fw' Lw Fw2 D' Lw2 Fw2 Uw D2 R' Lw L' B Uw' D Rw' L' U2 Rw D'");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [0, 2, 4, 1, 0],
                        [5, 0, 5, 0, 4],
                        [2, 5, 0, 1, 0],
                        [4, 2, 3, 4, 4],
                        [3, 5, 0, 3, 5]
                    ],
                    [
                        [2, 1, 5, 1, 1],
                        [3, 1, 1, 0, 5],
                        [5, 4, 1, 0, 5],
                        [4, 1, 2, 4, 0],
                        [0, 3, 2, 5, 4]
                    ],
                    [
                        [2, 2, 0, 3, 5],
                        [2, 1, 2, 3, 0],
                        [4, 0, 2, 3, 3],
                        [2, 3, 0, 0, 0],
                        [1, 3, 5, 3, 2]
                    ],
                    [
                        [0, 1, 3, 2, 1],
                        [1, 2, 5, 5, 3],
                        [3, 4, 3, 3, 2],
                        [0, 5, 0, 3, 4],
                        [2, 2, 3, 1, 5]
                    ],
                    [
                        [3, 4, 1, 4, 1],
                        [0, 2, 3, 4, 3],
                        [1, 1, 4, 5, 1],
                        [0, 1, 4, 5, 1],
                        [4, 2, 1, 5, 4]
                    ],
                    [
                        [5, 2, 0, 5, 4],
                        [5, 3, 4, 2, 0],
                        [4, 2, 5, 1, 4],
                        [5, 5, 2, 4, 4],
                        [3, 0, 2, 1, 3]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<FiveByFive>::new();

            cube.apply_scramble("Lw2 Uw' D2 R2 Lw' B Dw U F' Dw B2 U2 Rw' D2 Uw' Rw D' B2 D Uw2 L Uw2 L' Bw' Rw2 B2 L' Lw' F D2 Rw L Bw L2 Rw2 B2 Dw2 B' U F L2 Uw2 Bw' Uw2 Rw L Fw Rw D' U B' Fw2 Rw2 Lw2 U' Fw Lw D' R2 F'");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [0, 3, 5, 4, 3],
                        [1, 0, 2, 3, 2],
                        [3, 0, 0, 3, 1],
                        [5, 5, 0, 4, 5],
                        [3, 2, 4, 4, 0]
                    ],
                    [
                        [5, 2, 5, 5, 4],
                        [3, 1, 5, 4, 3],
                        [2, 1, 1, 4, 1],
                        [2, 5, 4, 0, 5],
                        [4, 3, 3, 5, 1]
                    ],
                    [
                        [3, 5, 2, 0, 2],
                        [4, 2, 4, 0, 5],
                        [5, 1, 2, 1, 5],
                        [1, 3, 1, 5, 4],
                        [1, 1, 1, 1, 2]
                    ],
                    [
                        [4, 2, 4, 4, 2],
                        [2, 2, 0, 2, 0],
                        [2, 2, 3, 2, 0],
                        [1, 1, 5, 3, 0],
                        [5, 3, 1, 0, 5]
                    ],
                    [
                        [3, 3, 2, 3, 1],
                        [2, 4, 3, 3, 3],
                        [0, 4, 4, 3, 3],
                        [4, 1, 2, 0, 0],
                        [0, 1, 4, 4, 2]
                    ],
                    [
                        [5, 0, 4, 1, 0],
                        [0, 1, 5, 2, 1],
                        [3, 5, 5, 0, 0],
                        [4, 5, 3, 4, 5],
                        [4, 2, 0, 0, 1]
                    ]
                ])
            );
            {
                let mut cube = Cube::<FiveByFive>::new();

                cube.apply_scramble("Dw F2 Lw2 F2 Bw2 Rw2 Dw' R Rw' Uw Dw' Bw2 Dw2 Lw2 Fw' D2 F2 R2 L Fw' Dw2 L B Fw R' Bw2 B F2 L2 Lw' B2 Uw2 U R2 B' F Bw R' B Dw2 Lw' Uw2 Fw B' Bw' L2 B' Rw' L R' B2 U2 Fw F' L R' Fw2 U' R2 Fw2");

                assert_eq!(
                    cube.state,
                    arr3(&[
                        [
                            [5, 2, 1, 2, 2],
                            [2, 1, 4, 3, 3],
                            [0, 0, 0, 3, 2],
                            [1, 4, 0, 0, 3],
                            [4, 5, 5, 5, 2]
                        ],
                        [
                            [5, 0, 5, 5, 1],
                            [0, 2, 3, 5, 1],
                            [4, 0, 1, 2, 5],
                            [5, 2, 4, 1, 1],
                            [0, 0, 5, 3, 3]
                        ],
                        [
                            [4, 2, 4, 2, 1],
                            [4, 0, 1, 4, 4],
                            [2, 5, 2, 1, 1],
                            [2, 5, 5, 5, 3],
                            [1, 4, 2, 1, 3]
                        ],
                        [
                            [0, 4, 0, 1, 2],
                            [1, 0, 4, 1, 5],
                            [4, 2, 3, 2, 3],
                            [0, 0, 0, 4, 4],
                            [0, 1, 1, 2, 3]
                        ],
                        [
                            [2, 0, 0, 5, 5],
                            [3, 3, 5, 5, 3],
                            [3, 4, 4, 1, 0],
                            [3, 1, 5, 2, 0],
                            [5, 3, 4, 1, 3]
                        ],
                        [
                            [0, 4, 3, 2, 1],
                            [4, 2, 3, 4, 0],
                            [1, 1, 5, 2, 3],
                            [4, 3, 3, 3, 5],
                            [4, 0, 2, 5, 4]
                        ]
                    ])
                );
            }
        }
        {
            let mut cube = Cube::<FiveByFive>::new();

            cube.apply_scramble("B Fw' Uw' L2 Bw' Fw2 D U2 Rw' R' Lw2 B Rw' Dw' Lw' Bw R2 L2 Rw Fw2 Uw2 D R2 Lw D' Fw Uw' U' R F2 D Dw2 F' B' Fw D2 Fw' B2 Rw R' Lw' Dw R' B' Fw2 Lw2 D2 F2 Fw Lw2 Dw F Bw' B' L2 Rw Fw' B2 L' D2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [1, 0, 4, 1, 4],
                        [3, 0, 0, 1, 0],
                        [1, 4, 0, 0, 4],
                        [0, 4, 0, 4, 5],
                        [0, 0, 5, 5, 3]
                    ],
                    [
                        [1, 2, 2, 0, 5],
                        [1, 2, 3, 5, 3],
                        [4, 3, 1, 4, 0],
                        [1, 5, 3, 4, 2],
                        [5, 3, 1, 2, 3]
                    ],
                    [
                        [4, 1, 0, 3, 2],
                        [2, 1, 3, 1, 4],
                        [0, 2, 2, 2, 3],
                        [5, 0, 5, 4, 1],
                        [1, 2, 2, 3, 2]
                    ],
                    [
                        [0, 2, 3, 5, 0],
                        [4, 2, 1, 3, 1],
                        [3, 1, 3, 1, 5],
                        [4, 2, 2, 3, 1],
                        [2, 5, 2, 2, 0]
                    ],
                    [
                        [5, 3, 4, 4, 4],
                        [4, 5, 2, 0, 4],
                        [0, 4, 4, 1, 3],
                        [3, 3, 4, 2, 0],
                        [4, 4, 5, 5, 1]
                    ],
                    [
                        [3, 1, 1, 0, 2],
                        [5, 0, 5, 5, 5],
                        [5, 5, 5, 0, 2],
                        [4, 3, 5, 1, 2],
                        [5, 3, 1, 0, 3]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<FiveByFive>::new();

            cube.apply_scramble("F' U2 Fw' B Dw' L' Rw R2 Bw L2 Bw2 Uw' Dw Fw2 F2 U2 Dw2 Uw' F' Dw L Bw' Dw F' Dw' Bw R2 L2 U B2 Dw2 Lw F2 B U Uw Dw F' Lw' R2 Fw Bw2 Uw2 Rw U' R Uw Bw' Lw Bw Fw2 Dw2 B' L' Bw R' Fw2 B Lw2 U'");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [3, 3, 3, 4, 0],
                        [3, 0, 1, 1, 5],
                        [3, 5, 0, 4, 0],
                        [4, 3, 4, 4, 2],
                        [4, 4, 5, 5, 0]
                    ],
                    [
                        [0, 2, 3, 0, 2],
                        [5, 0, 0, 2, 0],
                        [2, 3, 1, 3, 5],
                        [1, 0, 3, 5, 1],
                        [3, 5, 2, 5, 5]
                    ],
                    [
                        [5, 1, 4, 0, 4],
                        [2, 1, 1, 2, 5],
                        [4, 4, 2, 5, 1],
                        [1, 4, 1, 5, 3],
                        [1, 0, 0, 0, 2]
                    ],
                    [
                        [5, 4, 1, 2, 1],
                        [3, 2, 2, 3, 3],
                        [3, 4, 3, 5, 5],
                        [3, 5, 2, 0, 3],
                        [1, 5, 1, 0, 3]
                    ],
                    [
                        [2, 4, 4, 1, 3],
                        [2, 4, 1, 3, 2],
                        [0, 0, 4, 0, 0],
                        [3, 3, 2, 1, 1],
                        [5, 5, 5, 2, 2]
                    ],
                    [
                        [1, 1, 4, 0, 4],
                        [4, 4, 2, 5, 2],
                        [2, 3, 5, 5, 2],
                        [4, 2, 0, 1, 4],
                        [4, 1, 1, 0, 0]
                    ]
                ])
            );
        }
    }

    #[test]
    fn six_by_six() {
        {
            let mut cube = Cube::<SixBySix>::new();

            cube.apply_scramble("Fw2 U' Bw' Lw Uw' B L 3Fw2 Dw' 3Fw2 F R' Uw B2 Rw' B' L' U2 Lw Fw Rw Fw2 R' D' Rw 3Rw Uw2 F Rw' Lw2 R2 Dw' Fw2 D2 Lw R' Fw2 D R' 3Fw' Lw' Bw2 D2 3Fw 3Rw' R 3Fw L2 Rw' 3Fw2 B 3Rw2 R' 3Fw2 3Rw2 3Uw' 3Fw' B Rw' 3Rw2 Lw2 Bw Dw R' Lw B2 L F Bw Uw' Fw2 3Fw2 U2 Dw2 Rw 3Uw' 3Fw' 3Uw2 3Fw Dw'");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [B, B, F, D, L, D],
                        [F, B, B, U, B, F],
                        [F, U, L, D, R, L],
                        [L, U, L, F, B, L],
                        [B, R, F, R, D, B],
                        [F, D, U, U, B, F]
                    ],
                    [
                        [B, L, U, D, R, B],
                        [F, R, D, L, F, F],
                        [B, F, R, F, B, F],
                        [B, D, L, B, R, R],
                        [D, R, R, L, U, R],
                        [F, U, U, R, D, R]
                    ],
                    [
                        [R, L, B, B, L, D],
                        [F, U, B, D, F, U],
                        [F, U, U, B, R, D],
                        [B, F, F, F, B, D],
                        [D, F, F, U, D, U],
                        [U, R, B, D, L, R]
                    ],
                    [
                        [U, R, L, R, F, D],
                        [U, B, U, L, U, L],
                        [F, B, U, R, F, L],
                        [B, B, B, B, D, L],
                        [R, L, L, F, U, B],
                        [B, D, B, R, U, L]
                    ],
                    [
                        [F, U, U, R, B, U],
                        [L, D, F, R, L, R],
                        [U, F, D, D, R, F],
                        [D, L, R, R, U, R],
                        [F, F, D, D, B, L],
                        [R, D, L, R, U, L]
                    ],
                    [
                        [L, D, R, L, D, L],
                        [R, D, U, D, L, F],
                        [U, R, D, U, L, U],
                        [D, L, U, L, B, D],
                        [R, L, L, D, R, U],
                        [D, B, F, F, B, U]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SixBySix>::new();

            cube.apply_scramble("D' L' B' Fw 3Rw Lw2 Bw L Bw2 D' Bw2 L2 3Fw2 U2 3Fw' Bw2 F R' Dw' R B2 L Lw F Fw2 3Uw2 Dw' Fw' Lw' R D' L2 Bw F2 R2 U D2 R2 L' Rw B2 Rw B' Rw Lw2 Uw2 Lw Uw' B Fw2 Uw Rw' 3Uw Uw' Lw' U F2 Rw2 D Dw F2 Fw' 3Uw2 Uw2 L2 Uw2 F2 U2 R2 3Uw2 Uw' U' F2 Uw Fw' F' D Uw2 Bw2 D");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [R, L, F, R, D, D],
                        [B, B, D, U, F, R],
                        [R, D, U, U, L, U],
                        [R, U, U, D, L, L],
                        [F, R, U, B, F, F],
                        [L, L, D, F, L, R]
                    ],
                    [
                        [D, B, B, F, B, B],
                        [U, D, B, R, B, R],
                        [B, U, L, B, U, B],
                        [U, B, L, B, F, R],
                        [R, F, F, F, F, D],
                        [R, R, U, R, U, B]
                    ],
                    [
                        [F, B, L, B, L, U],
                        [R, R, R, U, L, R],
                        [B, B, R, D, D, D],
                        [B, U, L, R, F, D],
                        [F, D, D, F, B, U],
                        [F, U, U, R, F, U]
                    ],
                    [
                        [L, B, D, B, F, U],
                        [F, L, F, R, D, F],
                        [F, L, F, B, L, L],
                        [U, R, F, F, D, L],
                        [U, L, B, D, D, D],
                        [F, R, F, F, L, D]
                    ],
                    [
                        [R, L, R, F, L, B],
                        [B, B, R, F, L, B],
                        [L, L, R, D, L, F],
                        [D, L, R, D, B, U],
                        [D, U, R, R, R, F],
                        [L, L, U, L, D, D]
                    ],
                    [
                        [L, U, L, D, D, F],
                        [U, R, U, L, U, D],
                        [L, B, B, F, B, U],
                        [D, D, L, U, D, R],
                        [U, U, R, F, U, R],
                        [B, B, D, B, D, U]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SixBySix>::new();

            cube.apply_scramble("Rw2 Lw F2 Uw 3Uw R' Fw D2 R2 F2 Bw2 B L' U2 Rw' 3Fw2 L' 3Uw2 B L2 Rw R' 3Fw2 Uw Bw Rw Uw2 L2 R2 3Rw2 3Uw2 L2 B2 Lw' Rw' 3Fw2 Bw B' Fw Rw2 D' F L2 Rw 3Uw' Bw' Lw2 R2 3Fw2 B 3Rw' Lw B' 3Rw' Lw2 F Uw Rw Dw 3Rw2 Rw Uw F' Uw Dw' L' Uw R2 D2 3Fw2 Fw2 B' L' 3Uw Fw' Dw' L Dw' F Bw'");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [U, L, U, F, U, D],
                        [B, D, U, U, R, D],
                        [D, R, R, D, R, R],
                        [D, D, F, L, B, L],
                        [R, U, L, L, U, L],
                        [R, F, L, L, B, D]
                    ],
                    [
                        [L, L, F, L, F, B],
                        [D, L, F, D, D, L],
                        [D, F, D, U, B, R],
                        [D, U, U, U, D, B],
                        [B, R, B, B, B, D],
                        [R, D, D, B, F, L]
                    ],
                    [
                        [U, F, L, R, U, F],
                        [U, L, L, R, L, D],
                        [F, D, L, L, F, B],
                        [F, R, F, B, R, R],
                        [U, R, F, B, B, D],
                        [R, R, F, F, B, F]
                    ],
                    [
                        [B, B, R, F, R, B],
                        [R, U, B, B, F, R],
                        [L, F, F, B, U, U],
                        [B, D, L, B, L, D],
                        [D, D, F, L, D, R],
                        [R, B, U, L, U, D]
                    ],
                    [
                        [B, D, L, U, L, D],
                        [L, F, R, U, F, R],
                        [U, L, D, D, D, B],
                        [R, D, U, R, U, B],
                        [R, F, B, R, B, U],
                        [F, B, B, U, L, F]
                    ],
                    [
                        [L, U, D, R, F, U],
                        [F, L, F, L, R, F],
                        [B, U, F, B, F, F],
                        [U, L, R, R, R, U],
                        [F, U, D, U, B, L],
                        [L, B, R, D, U, U]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SixBySix>::new();

            cube.apply_scramble("Lw' Fw' Bw 3Uw' L' B U' Lw' U2 L2 U2 R U2 L F' Bw 3Rw Lw Dw' Uw' B F' 3Rw' Bw' Rw2 Dw' R Fw L' 3Fw2 R 3Rw2 L2 Dw2 3Rw' R2 3Uw2 B' U2 Lw 3Fw' Rw2 Dw U' L' 3Fw2 L U2 L2 3Uw2 Rw 3Rw2 3Fw L Uw2 3Fw2 Dw Uw2 Rw 3Rw2 3Fw2 R' Bw Lw2 Uw2 Dw Lw2 3Rw' 3Uw Bw2 3Rw2 Fw' U2 Lw2 3Rw2 Rw2 Bw2 U' D R", );

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [B, F, L, D, L, L],
                        [F, U, R, F, F, U],
                        [R, R, D, B, B, B],
                        [D, F, R, L, L, R],
                        [L, L, U, L, R, U],
                        [F, L, U, L, F, D]
                    ],
                    [
                        [R, L, F, D, L, B],
                        [B, F, B, D, B, B],
                        [F, F, U, R, D, B],
                        [D, D, F, U, U, U],
                        [R, D, B, B, L, R],
                        [U, U, F, R, L, D]
                    ],
                    [
                        [F, R, L, B, U, L],
                        [R, F, F, R, B, D],
                        [L, R, F, F, D, U],
                        [D, L, L, L, D, L],
                        [L, B, B, B, L, D],
                        [F, R, F, F, F, D]
                    ],
                    [
                        [F, U, R, B, B, L],
                        [U, R, R, U, D, B],
                        [B, F, D, B, R, B],
                        [F, U, R, B, B, F],
                        [R, U, D, F, U, B],
                        [B, D, U, U, D, L]
                    ],
                    [
                        [D, U, U, R, L, R],
                        [R, B, F, L, D, D],
                        [L, B, D, R, D, R],
                        [F, R, U, D, L, B],
                        [B, R, L, L, D, D],
                        [R, F, D, D, D, B]
                    ],
                    [
                        [U, U, R, L, F, U],
                        [F, U, U, U, L, F],
                        [U, D, L, U, L, U],
                        [B, U, F, B, R, D],
                        [B, R, F, U, F, D],
                        [R, B, R, L, R, U]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SixBySix>::new();

            cube.apply_scramble(
                "F2 U F' Rw' B2 3Fw2 L' 3Rw' B' Rw F' Fw 3Rw2 3Uw2 R Uw2 Rw2 3Fw2 Bw2 F' U Fw2 L2 U2 Uw Lw Fw2 U' 3Rw Uw2 3Uw2 B Dw D' F2 L' D' L R B2 Dw2 F2 R Fw' R' F2 3Rw' Dw Uw2 Lw Uw2 D' F2 B 3Fw2 Rw2 U2 D L' Fw 3Fw' D' 3Rw' D' Uw U' B Dw' B Fw2 3Rw' 3Uw2 R' U' R2 3Uw D2 Uw 3Fw2 Fw"
);

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [B, R, B, B, F, L],
                        [R, F, F, R, F, L],
                        [L, F, F, U, D, F],
                        [F, F, U, F, B, F],
                        [F, D, B, L, R, B],
                        [L, L, L, U, F, U]
                    ],
                    [
                        [R, R, D, R, D, F],
                        [B, U, R, R, R, L],
                        [D, D, D, F, D, L],
                        [D, U, D, B, L, U],
                        [B, B, R, R, B, U],
                        [R, L, F, B, D, R]
                    ],
                    [
                        [U, B, U, R, F, D],
                        [F, L, D, U, L, D],
                        [F, U, R, F, R, B],
                        [L, F, L, B, U, D],
                        [F, F, B, B, U, R],
                        [L, B, L, R, B, U]
                    ],
                    [
                        [F, D, R, B, U, B],
                        [B, R, L, R, D, U],
                        [D, L, L, D, F, L],
                        [U, B, R, R, F, D],
                        [F, F, U, L, U, D],
                        [F, R, F, R, U, B]
                    ],
                    [
                        [D, U, F, U, D, B],
                        [R, R, D, F, D, U],
                        [B, L, L, D, U, U],
                        [R, B, B, B, D, R],
                        [B, B, F, L, B, U],
                        [R, U, U, D, D, L]
                    ],
                    [
                        [D, R, F, R, F, U],
                        [D, L, U, D, L, R],
                        [U, D, L, U, B, B],
                        [D, U, R, U, L, B],
                        [L, D, B, R, U, L],
                        [F, L, L, L, L, D]
                    ]
                ])
            );
        }
    }
    #[test]
    fn seven_by_seven() {
        {
            let mut cube = Cube::<SevenBySeven>::new();

            cube.apply_scramble("D 3Fw Bw2 U' Bw R2 3Bw L' 3Uw R Rw2 3Lw' L' Uw' 3Lw2 3Dw2 B2 L Rw' 3Dw B' U2 F' B U 3Rw' R F' Dw' 3Rw' 3Fw Rw2 R2 3Lw' 3Uw 3Rw2 F 3Dw Rw2 Lw' Fw' 3Bw2 U' L' 3Bw2 U2 Uw 3Bw2 Lw' U 3Lw' B Fw2 D2 3Lw' 3Bw Uw2 B D Lw 3Rw2 3Lw' Bw' Fw' 3Fw' 3Lw Dw' 3Uw Rw2 U2 B' Bw Rw' Fw' R2 F 3Rw B2 3Uw Dw' F' 3Lw2 D2 3Lw' 3Uw' Uw2 Rw 3Fw' D' Dw2 U' Fw Bw' 3Uw' Lw2 3Dw' Uw' R Rw 3Rw2");

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [L, F, R, R, L, F, B],
                        [F, D, F, D, D, L, R],
                        [B, U, L, L, L, D, F],
                        [L, F, B, R, F, U, B],
                        [U, U, U, L, L, L, U],
                        [D, U, F, L, L, L, L],
                        [U, R, F, R, L, R, R]
                    ],
                    [
                        [F, L, B, D, F, D, U],
                        [B, R, L, B, F, U, U],
                        [B, R, D, B, D, D, D],
                        [R, B, U, U, U, B, F],
                        [B, F, R, F, F, F, F],
                        [B, B, B, U, D, F, L],
                        [F, R, B, U, F, B, B]
                    ],
                    [
                        [L, F, R, F, L, L, D],
                        [L, D, L, R, U, D, D],
                        [U, D, U, U, L, U, L],
                        [L, L, D, F, R, U, U],
                        [R, B, F, B, B, D, B],
                        [L, F, U, F, B, B, R],
                        [B, B, R, B, U, L, F]
                    ],
                    [
                        [D, U, D, U, D, D, U],
                        [R, F, R, D, B, B, D],
                        [R, B, R, D, D, L, R],
                        [F, R, L, L, F, B, F],
                        [U, R, B, R, U, L, D],
                        [R, D, B, R, R, R, B],
                        [D, D, D, B, B, R, U]
                    ],
                    [
                        [R, D, D, U, R, F, R],
                        [F, L, U, R, U, R, B],
                        [U, R, R, D, F, B, L],
                        [D, L, U, D, B, D, D],
                        [F, F, F, L, U, F, D],
                        [F, F, L, F, R, U, U],
                        [R, U, U, R, U, F, F]
                    ],
                    [
                        [L, L, D, L, L, U, L],
                        [D, U, F, D, D, B, U],
                        [R, R, R, F, B, B, B],
                        [L, F, D, B, R, U, D],
                        [F, D, D, R, B, R, F],
                        [U, R, L, L, U, L, B],
                        [D, U, L, B, L, B, B]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SevenBySeven>::new();

            cube.apply_scramble(
                "D2 Fw2 U2 3Rw 3Dw D' 3Uw' 3Bw Bw' Rw2 D U 3Uw' 3Lw F U 3Lw2 3Dw' 3Bw' 3Dw2 Fw2 3Dw' Bw2 R' Dw' R' 3Rw2 3Bw' Fw2 R2 3Dw2 Bw2 3Rw2 U 3Uw Fw' Uw2 3Uw' 3Dw Fw' Dw2 B Rw' 3Dw' B U' 3Dw2 Fw' D' 3Fw U 3Rw2 D 3Uw Dw 3Dw L2 3Lw Rw 3Rw' Dw' U Rw' L Fw2 3Fw' L' D' R2 U' Bw D' Rw2 L 3Dw' 3Fw' U Lw L Bw Uw2 L' B2 Fw' R' F' 3Dw2 3Lw' U' 3Rw2 F 3Uw' Lw 3Rw2 3Lw D' B2 3Fw D Bw2",
);

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [B, D, U, D, L, L, B],
                        [U, D, R, L, L, R, U],
                        [F, B, F, U, R, U, F],
                        [R, D, F, R, L, R, L],
                        [R, R, L, B, F, U, U],
                        [R, U, L, B, B, B, B],
                        [U, R, B, D, R, R, F]
                    ],
                    [
                        [D, F, R, R, F, F, L],
                        [F, D, F, U, F, B, U],
                        [R, U, D, D, L, D, D],
                        [U, U, B, U, B, L, F],
                        [F, F, L, R, R, L, R],
                        [R, L, B, F, L, R, F],
                        [U, D, R, R, U, D, U]
                    ],
                    [
                        [F, F, D, D, L, L, R],
                        [U, L, R, D, D, U, L],
                        [U, D, D, F, B, L, U],
                        [F, F, R, F, L, B, U],
                        [F, D, B, F, R, B, U],
                        [B, R, B, U, D, D, U],
                        [F, L, B, R, U, U, B]
                    ],
                    [
                        [R, R, B, L, L, B, L],
                        [D, F, F, R, R, U, B],
                        [L, F, D, D, B, R, B],
                        [D, L, U, L, F, F, U],
                        [D, U, F, L, D, F, R],
                        [D, U, R, D, B, L, D],
                        [F, B, D, U, L, F, L]
                    ],
                    [
                        [D, B, D, B, B, R, L],
                        [U, L, U, L, F, D, B],
                        [D, R, L, B, U, F, R],
                        [B, D, R, D, U, B, L],
                        [B, L, F, L, U, D, D],
                        [L, B, R, U, B, R, D],
                        [R, F, B, L, F, F, R]
                    ],
                    [
                        [D, L, L, F, F, L, B],
                        [B, F, D, B, U, F, L],
                        [U, D, R, D, U, U, B],
                        [F, R, R, B, D, R, B],
                        [L, B, U, U, B, U, F],
                        [D, F, L, F, L, B, R],
                        [U, R, D, B, L, U, D]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SevenBySeven>::new();

            cube.apply_scramble(
                "3Bw2 Rw F Lw' 3Lw' 3Bw 3Dw Uw Bw Fw Rw 3Bw2 F' 3Lw Dw2 3Rw F Rw2 D Fw' U F' Fw 3Uw' Lw' L2 D2 Fw' D' Bw2 Fw Uw U Lw R' 3Rw' Rw' 3Fw' 3Rw' Lw' Dw' L' 3Uw D2 B2 3Rw' F2 D' Lw F2 D Dw2 Uw L2 F' 3Fw' Uw' Rw' F2 Dw2 3Rw 3Dw' R B2 U2 3Rw2 Lw' Uw' 3Bw Fw2 B2 Bw 3Rw Uw' D2 R2 3Lw2 3Dw 3Rw F' Lw Dw Bw 3Bw Dw 3Lw L' B 3Lw' B' 3Dw' Bw2 L' Rw2 D' 3Uw2 Lw B' F Bw2",
);

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [D, R, B, R, L, R, D],
                        [F, B, B, F, D, U, U],
                        [F, D, L, R, R, B, U],
                        [L, L, F, R, F, F, F],
                        [U, D, B, B, B, B, D],
                        [F, F, D, B, L, D, L],
                        [U, U, L, D, U, R, B]
                    ],
                    [
                        [R, F, D, R, B, F, B],
                        [F, L, U, U, L, F, U],
                        [D, R, L, D, F, R, B],
                        [U, F, F, U, L, B, F],
                        [R, F, B, F, U, U, R],
                        [L, L, L, L, R, L, U],
                        [D, U, L, B, L, L, B]
                    ],
                    [
                        [L, B, U, L, F, D, R],
                        [R, U, L, R, F, D, U],
                        [U, R, D, R, U, F, R],
                        [F, L, L, F, R, U, U],
                        [R, U, R, R, F, D, F],
                        [F, R, F, F, L, B, D],
                        [B, L, U, B, F, D, F]
                    ],
                    [
                        [D, L, L, R, B, B, F],
                        [D, F, B, R, F, B, B],
                        [B, B, R, L, D, D, R],
                        [F, D, U, L, D, D, D],
                        [B, F, L, B, L, U, D],
                        [B, R, F, R, R, R, U],
                        [L, R, R, B, D, F, L]
                    ],
                    [
                        [U, B, L, U, R, B, R],
                        [R, R, D, U, B, D, L],
                        [L, L, F, B, U, F, F],
                        [D, L, U, D, U, R, L],
                        [D, B, U, B, D, R, R],
                        [D, U, R, B, R, U, F],
                        [U, L, U, B, B, R, U]
                    ],
                    [
                        [L, D, D, D, F, L, F],
                        [B, L, U, D, U, F, R],
                        [B, B, B, D, F, L, L],
                        [L, D, U, B, L, U, U],
                        [F, D, D, D, R, U, U],
                        [U, B, L, B, U, D, D],
                        [R, B, D, R, F, D, F]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SevenBySeven>::new();

            cube.apply_scramble(
                "3Rw B' Uw' Dw F2 Rw' Fw2 B' Dw 3Rw' U' 3Lw2 3Dw2 3Uw2 Uw' Rw2 D L' U2 Lw2 3Dw Fw2 3Rw' Rw2 3Fw 3Dw2 3Rw' Dw2 U' 3Lw' F D Rw 3Dw' 3Fw 3Dw Bw' Lw F' Dw F2 L Uw2 Rw2 Lw Fw' F B' U Dw 3Dw2 D' 3Rw' Dw2 Lw' Rw 3Bw Dw' Rw B U' Rw2 B 3Rw' Bw2 Rw2 L B2 U' R' Fw Dw' D 3Dw' Rw' Dw D Rw2 Fw' 3Bw 3Fw2 Rw 3Uw' U' 3Rw 3Uw F' 3Lw' U2 3Rw' F2 R 3Rw Dw F B' Dw2 F2 Rw2 3Rw'",
);

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [R, R, D, B, B, R, B],
                        [U, F, U, R, B, D, U],
                        [D, U, B, U, L, U, B],
                        [D, L, F, R, R, B, R],
                        [L, L, B, R, F, D, U],
                        [D, L, F, F, U, R, B],
                        [B, L, B, L, B, D, U]
                    ],
                    [
                        [D, D, U, D, D, F, R],
                        [L, U, D, R, R, L, F],
                        [L, L, B, D, L, D, U],
                        [U, U, D, U, B, B, L],
                        [F, R, F, D, R, B, L],
                        [B, B, L, D, F, U, U],
                        [U, F, R, F, U, R, D]
                    ],
                    [
                        [L, U, D, D, F, D, B],
                        [F, L, R, R, B, B, L],
                        [R, R, F, F, D, F, F],
                        [R, F, L, F, L, B, R],
                        [R, L, L, L, R, F, F],
                        [F, U, U, U, B, U, B],
                        [D, R, D, D, B, R, D]
                    ],
                    [
                        [L, D, F, B, U, R, B],
                        [B, F, B, L, U, B, L],
                        [R, U, R, U, U, F, F],
                        [L, L, R, L, B, F, B],
                        [L, L, U, U, D, L, F],
                        [B, B, D, D, D, D, D],
                        [R, L, F, R, U, L, R]
                    ],
                    [
                        [F, U, L, L, D, B, L],
                        [D, D, R, F, F, D, B],
                        [R, B, D, R, B, D, D],
                        [U, U, B, D, F, U, F],
                        [D, U, R, F, L, D, U],
                        [F, L, L, L, B, R, R],
                        [F, U, B, F, L, B, L]
                    ],
                    [
                        [U, R, R, B, L, F, F],
                        [L, F, R, R, D, R, L],
                        [L, R, U, B, U, F, B],
                        [F, D, D, B, L, D, U],
                        [R, L, D, U, F, F, B],
                        [U, F, R, B, B, R, U],
                        [F, D, U, U, R, F, U]
                    ]
                ])
            );
        }
        {
            let mut cube = Cube::<SevenBySeven>::new();

            cube.apply_scramble(
                "3Lw2 Fw 3Rw2 Dw' Uw R' Dw2 Bw2 Uw Dw Lw2 Fw' L Bw 3Rw 3Uw2 Uw' F' U 3Uw' 3Rw2 3Dw' 3Bw2 Dw L Lw D Fw' 3Uw2 Fw' Dw' 3Dw2 Rw2 F' Lw Rw Fw' 3Rw' 3Fw' Rw2 B' 3Uw 3Dw' Bw2 Fw2 U2 F L' U2 B' Bw Fw2 3Uw D' F' 3Lw' R2 3Bw2 Fw' 3Rw' Dw' U' 3Dw2 Rw2 3Dw' 3Uw 3Rw' Fw2 3Uw' L 3Rw' Bw 3Bw Dw2 Fw R2 D2 3Rw' 3Dw B Fw Uw F2 Uw2 3Bw' Dw' 3Dw' 3Bw R2 3Bw 3Rw' Uw2 R2 3Uw2 U2 3Dw2 3Lw 3Rw' U2 Rw"
);

            assert_eq!(
                cube.state,
                arr3(&[
                    [
                        [R, U, R, U, L, L, B],
                        [R, L, D, B, F, B, U],
                        [L, L, F, R, D, F, L],
                        [R, F, L, R, L, R, F],
                        [U, L, F, U, D, R, U],
                        [F, L, U, D, U, D, F],
                        [R, B, D, L, F, R, L]
                    ],
                    [
                        [L, L, D, B, F, L, D],
                        [B, B, B, F, D, F, U],
                        [F, D, R, L, R, U, D],
                        [D, F, U, U, B, L, F],
                        [F, B, F, R, R, U, B],
                        [D, R, F, D, R, F, R],
                        [L, U, L, F, U, F, F]
                    ],
                    [
                        [D, B, B, R, B, R, D],
                        [F, R, U, B, D, U, B],
                        [U, B, U, D, L, R, U],
                        [B, D, B, F, R, F, B],
                        [R, D, F, D, B, R, R],
                        [B, R, F, L, F, U, D],
                        [L, D, R, D, B, R, B]
                    ],
                    [
                        [F, L, D, L, L, R, B],
                        [D, U, F, U, B, L, D],
                        [B, R, U, R, D, U, R],
                        [L, R, U, L, D, L, D],
                        [B, L, L, F, R, U, F],
                        [D, F, F, U, F, U, L],
                        [R, L, D, R, F, U, U]
                    ],
                    [
                        [B, L, F, R, U, U, U],
                        [F, F, L, D, B, D, R],
                        [D, U, U, U, B, R, B],
                        [U, U, B, D, F, R, U],
                        [R, L, D, L, U, R, U],
                        [F, D, B, B, B, R, D],
                        [F, F, F, F, L, F, F]
                    ],
                    [
                        [R, D, U, U, R, U, D],
                        [B, B, R, L, L, B, B],
                        [B, L, L, B, B, D, D],
                        [D, B, D, B, F, R, B],
                        [L, B, L, F, B, D, R],
                        [R, L, L, U, D, D, B],
                        [U, U, D, L, L, L, U]
                    ]
                ])
            );
        }
    }
}
