use crate::puzzles::{ColorSchemes, Puzzle};
use crate::utils::color::Color;
use crate::utils::maths::rotate_2d_matrix;
use lazy_static::lazy_static;
use ndarray::{s, Array1, Array3, ArrayView2};
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use svg::node::element::{Rectangle, SVG};
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
        if scramble.trim().is_empty() {
            return;
        }

        let moves = scramble.split_ascii_whitespace();
        moves.for_each(|mv| self.apply_move(mv));
    }

    fn get_default_color_scheme(&self) -> ColorSchemes {
        ColorSchemes::Cube(DEFAULT_COLOR_SCHEME.clone())
    }

    fn draw(&self) -> SVG {
        let (width, height) = self.get_preferred_size();

        let mut svg = Document::new().set("viewBox", (0, 0, width, height));

        self.draw_cube(&mut svg);
        svg
    }
}

impl<T> Cube<T> {
    fn draw_cube(&self, g: &mut SVG) {
        let gap = self.gap;
        let size = self.size;
        let cubie_size = self.cubie_size;

        Cube::<T>::paint_cube_face(
            g,
            3 * gap + 2 * size * cubie_size,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state.slice(s![0, .., ..]),
            &self.color_scheme,
        );

        Cube::<T>::paint_cube_face(
            g,
            2 * gap + size * cubie_size,
            gap,
            size,
            cubie_size,
            &self.state.slice(s![1, .., ..]),
            &self.color_scheme,
        );

        Cube::<T>::paint_cube_face(
            g,
            2 * gap + size * cubie_size,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state.slice(s![2, .., ..]),
            &self.color_scheme,
        );

        Cube::<T>::paint_cube_face(
            g,
            gap,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state.slice(s![3, .., ..]),
            &self.color_scheme,
        );

        Cube::<T>::paint_cube_face(
            g,
            2 * gap + size * cubie_size,
            3 * gap + 2 * size * cubie_size,
            size,
            cubie_size,
            &self.state.slice(s![4, .., ..]),
            &self.color_scheme,
        );

        Cube::<T>::paint_cube_face(
            g,
            4 * gap + 3 * size * cubie_size,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state.slice(s![5, .., ..]),
            &self.color_scheme,
        );
    }

    fn paint_cube_face(
        svg: &mut SVG,
        x: usize,
        y: usize,
        size: usize,
        cubie_size: usize,
        face_colors: &ArrayView2<Face>,
        color_scheme: &HashMap<Face, Color>,
    ) {
        for row in 0..size {
            for col in 0..size {
                let x = x + col * cubie_size;
                let y = y + row * cubie_size;

                let color = if let Some(col) = color_scheme.get(&face_colors[[row, col]]) {
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

                svg.append(rect);
            }
        }
    }

    fn get_preferred_size(&self) -> (usize, usize) {
        let width = (self.size * self.cubie_size + self.gap) * 4 + self.gap;
        let height = (self.size * self.cubie_size + self.gap) * 3 + self.gap;

        (width, height)
    }

    fn apply_move(&mut self, mv: &str) {
        match mv {
            str if str.chars().last().unwrap_or('a').is_numeric() => {
                let mut chars = str.chars();
                chars.next_back();
                let mv = chars.as_str();

                self.apply_move(mv);
                self.apply_move(mv);
            }
            "R" | "Rw" => {
                let mut face = self.state.slice_mut(s![0, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), true));
                self.rotate_rl_layer([1, 2, 4, 5], [self.size - 1, 0]);
                if mv == "Rw" {
                    self.rotate_rl_layer([1, 2, 4, 5], [self.size - 2, 1]);
                }
            }
            "R'" | "Rw'" => {
                let mut face = self.state.slice_mut(s![0, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), false));
                self.rotate_rl_layer([4, 2, 1, 5], [self.size - 1, 0]);
                if mv == "Rw'" {
                    self.rotate_rl_layer([4, 2, 1, 5], [self.size - 2, 1]);
                }
            }
            "L" | "Lw" => {
                let mut face = self.state.slice_mut(s![3, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), true));
                self.rotate_rl_layer([4, 2, 1, 5], [0, self.size - 1]);
                if mv == "Lw" {
                    self.rotate_rl_layer([4, 2, 1, 5], [1, self.size - 2]);
                }
            }
            "L'" | "Lw'" => {
                let mut face = self.state.slice_mut(s![3, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), false));
                self.rotate_rl_layer([1, 2, 4, 5], [0, self.size - 1]);
                if mv == "Lw'" {
                    self.rotate_rl_layer([1, 2, 4, 5], [1, self.size - 2]);
                }
            }
            "U" | "Uw" => {
                let mut face = self.state.slice_mut(s![1, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), true));
                self.rotate_ud_layer([0, 5, 3, 2], 0);
                if mv == "Uw" {
                    self.rotate_ud_layer([0, 5, 3, 2], 1);
                }
            }
            "U'" | "Uw'" => {
                let mut face = self.state.slice_mut(s![1, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), false));
                self.rotate_ud_layer([0, 2, 3, 5], 0);
                if mv == "Uw'" {
                    self.rotate_ud_layer([0, 2, 3, 5], 1);
                }
            }
            "D" | "Dw" => {
                let mut face = self.state.slice_mut(s![4, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), true));
                self.rotate_ud_layer([0, 2, 3, 5], self.size - 1);
                if mv == "Dw" {
                    self.rotate_ud_layer([0, 2, 3, 5], self.size - 2);
                }
            }
            "D'" | "Dw'" => {
                let mut face = self.state.slice_mut(s![4, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), false));
                self.rotate_ud_layer([0, 5, 3, 2], self.size - 1);
                if mv == "Dw'" {
                    self.rotate_ud_layer([0, 5, 3, 2], self.size - 2);
                }
            }
            "F" | "Fw" => {
                let mut face = self.state.slice_mut(s![2, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), true));
                self.rotate_fb_layer([1, 3, 4, 0], 0);
                if mv == "Fw" {
                    self.rotate_fb_layer([1, 3, 4, 0], 1);
                }
            }
            "F'" | "Fw'" => {
                let mut face = self.state.slice_mut(s![2, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), false));
                self.rev_rotate_fb_layer([1, 0, 4, 3], 0);
                if mv == "Fw'" {
                    self.rev_rotate_fb_layer([1, 0, 4, 3], 1);
                }
            }
            "B" | "Bw" => {
                let mut face = self.state.slice_mut(s![5, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), true));
                self.rev_rotate_fb_layer([4, 3, 1, 0], 0);
                if mv == "Bw" {
                    self.rev_rotate_fb_layer([4, 3, 1, 0], 1);
                }
            }
            "B'" | "Bw'" => {
                let mut face = self.state.slice_mut(s![5, .., ..]);
                face.assign(&rotate_2d_matrix(&mut face.to_owned(), false));
                self.rotate_fb_layer([4, 0, 1, 3], 0);
                if mv == "Bw'" {
                    self.rotate_fb_layer([4, 0, 1, 3], 1);
                }
            }
            _ => {
                panic!()
                // log(("Error, movement not recognised: ".to_string() + mv).as_str());
            }
        }
    }

    fn rotate_rl_layer(&mut self, r: [usize; 4], side: [usize; 2]) {
        let tmp: Array1<Face> = self
            .state
            .slice(s![r[0], .., side[0]])
            .to_owned()
            .iter()
            .cloned()
            .rev()
            .collect();

        let b = self.state.slice(s![r[1], .., side[0]]).to_owned();
        self.state.slice_mut(s![r[0], .., side[0]]).assign(&b);

        let c = self.state.slice(s![r[2], .., side[0]]).to_owned();
        self.state.slice_mut(s![r[1], .., side[0]]).assign(&c);

        let d: Array1<Face> = self
            .state
            .slice(s![r[3], .., side[1]])
            .iter()
            .cloned()
            .rev()
            .collect();
        self.state.slice_mut(s![r[2], .., side[0]]).assign(&d);
        self.state.slice_mut(s![r[3], .., side[1]]).assign(&tmp);
    }

    fn rotate_ud_layer(&mut self, r: [usize; 4], side: usize) {
        let tmp = self.state.slice(s![r[0], side, ..]).to_owned();
        for i in 0..3 {
            let b = self.state.slice(s![r[i + 1], side, ..]).to_owned();
            self.state.slice_mut(s![r[i], side, ..]).assign(&b);
        }
        self.state.slice_mut(s![r[3], side, ..]).assign(&tmp);
    }

    fn rotate_fb_layer(&mut self, r: [usize; 4], layer: usize) {
        let tmp = self
            .state
            .slice(s![r[0], self.size - layer - 1, ..])
            .to_owned();

        let b: Array1<Face> = self
            .state
            .slice(s![r[1], .., self.size - layer - 1])
            .iter()
            .cloned()
            .rev()
            .collect();
        self.state
            .slice_mut(s![r[0], self.size - layer - 1, ..])
            .assign(&b);

        let c = self.state.slice(s![r[2], layer, ..]).to_owned();
        self.state
            .slice_mut(s![r[1], .., self.size - layer - 1])
            .assign(&c);

        let d: Array1<Face> = self
            .state
            .slice(s![r[3], .., layer])
            .iter()
            .cloned()
            .rev()
            .collect();
        self.state.slice_mut(s![r[2], layer, ..]).assign(&d);

        self.state.slice_mut(s![r[3], .., layer]).assign(&tmp);
    }

    fn rev_rotate_fb_layer(&mut self, r: [usize; 4], layer: usize) {
        let tmp = self
            .state
            .slice(s![r[0], self.size - layer - 1, ..])
            .to_owned();

        let b = self.state.slice(s![r[1], .., layer]).to_owned();
        self.state
            .slice_mut(s![r[0], self.size - layer - 1, ..])
            .assign(&b);

        for i in 0..self.size {
            self.state[[r[1], i, layer]] = self.state[[r[2], layer, self.size - i - 1]];
        }

        for j in 0..self.size {
            self.state[[r[2], layer, j]] = self.state[[r[3], j, self.size - layer - 1]];
        }

        for i in 0..self.size {
            self.state[[r[3], i, self.size - layer - 1]] = tmp[self.size - i - 1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
