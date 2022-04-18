use crate::color::Color;
use crate::dimension::Dimension;
use crate::element::Element;
use crate::log;
use crate::svg::Svg;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct CubePuzzle {
    pub size: usize,
    pub cubie_size: i32,
    pub gap: i32,
    pub image: Vec<Vec<Vec<usize>>>,
}

static FACES: [&str; 6] = ["R", "U", "F", "L", "D", "B"];

impl CubePuzzle {
    fn init_image(&mut self) {
        self.image = vec![vec![vec![0; self.size]; self.size]; 6];

        for face in 0..6 {
            for j in 0..self.size {
                for k in 0..self.size {
                    self.image[face][j][k] = face;
                }
            }
        }
    }
    pub fn set_cube(&mut self, t: &str) {
        match t {
            "333" | "OH" | "3BLD" => {
                self.size = 3;
            }
            "222" => {
                self.size = 2;
            }
            "444" => {
                self.size = 4;
            }
            "555" => {
                self.size = 5;
            }
            _ => {}
        }
        self.cubie_size = 10;
        self.gap = 2;
        self.init_image();
    }

    pub fn reset(&mut self) {
        self.init_image();
    }

    fn get_face_index(s: &str) -> usize {
        FACES.iter().position(|&x| x == s).unwrap()
    }

    pub fn default_color_scheme() -> &'static HashMap<&'static str, Color> {
        lazy_static! {
            static ref DEFAULT_COLOR_SCHEME: HashMap<&'static str, Color> = HashMap::from([
                ("B", Color::new_color("BLUE")),
                ("D", Color::new_color("YELLOW")),
                ("F", Color::new_color("GREEN")),
                ("L", Color::new_color("ORANGE")),
                ("R", Color::new_color("RED")),
                ("U", Color::new_color("WHITE")),
            ]);
        }

        return &DEFAULT_COLOR_SCHEME;
    }

    pub fn draw_cube(
        g: &mut Svg,
        state: &Vec<Vec<Vec<usize>>>,
        gap: i32,
        size: usize,
        cubie_size: i32,
        color_scheme: &HashMap<&str, Color>,
    ) {
        CubePuzzle::paint_cube_face(
            g,
            gap,
            2 * gap + size as i32 * cubie_size,
            size,
            cubie_size,
            &state[CubePuzzle::get_face_index("L")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            2 * gap + size as i32 * cubie_size,
            3 * gap + 2 * size as i32 * cubie_size,
            size,
            cubie_size,
            &state[CubePuzzle::get_face_index("D")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            4 * gap + 3 * size as i32 * cubie_size,
            2 * gap + size as i32 * cubie_size,
            size,
            cubie_size,
            &state[CubePuzzle::get_face_index("B")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            3 * gap + 2 * size as i32 * cubie_size,
            2 * gap + size as i32 * cubie_size,
            size,
            cubie_size,
            &state[CubePuzzle::get_face_index("R")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            2 * gap + size as i32 * cubie_size,
            gap,
            size,
            cubie_size,
            &state[CubePuzzle::get_face_index("U")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            2 * gap + size as i32 * cubie_size,
            2 * gap + size as i32 * cubie_size,
            size,
            cubie_size,
            &state[CubePuzzle::get_face_index("F")],
            color_scheme,
        );
    }

    pub fn paint_cube_face(
        g: &mut Svg,
        x: i32,
        y: i32,
        size: usize,
        cubie_size: i32,
        face_colors: &Vec<Vec<usize>>,
        color_scheme: &HashMap<&str, Color>,
    ) {
        for row in 0..size {
            for col in 0..size {
                let tempx = x + col as i32 * cubie_size;
                let tempy = y + row as i32 * cubie_size;

                let mut rect: Element = Element {
                    name: "".to_string(),
                    attributes: Default::default(),
                };
                rect.rectangle(
                    tempx as f32,
                    tempy as f32,
                    cubie_size as f32,
                    cubie_size as f32,
                );

                let col: Color = match color_scheme.get(FACES[face_colors[row][col]]) {
                    Some(x) => *x,
                    None => Color(0, 0, 0),
                };

                rect.set_fill(col);
                rect.set_stroke(Color::new_color("BLACK"));
                g.append_child(rect);
            }
        }
    }

    pub fn get_preferred_size(size: i32, cubie: i32, gap: i32) -> Dimension {
        let width = (size * cubie + gap) * 4 + gap;
        let height = (size * cubie + gap) * 3 + gap;

        return Dimension { width, height };
    }

    fn rotate_2d_matrix(size: usize, clockwise: bool, matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let mut rotated: Vec<Vec<usize>> = vec![vec![0; size]; size];

        for i in 0..size {
            for j in 0..size {
                if clockwise {
                    rotated[j][i] = matrix[size - i - 1][j];
                } else {
                    rotated[j][i] = matrix[i][size - j - 1];
                }
            }
        }
        return rotated;
    }

    fn rotate_rl_layer(
        r: [usize; 4],
        size: usize,
        side: [usize; 2],
        matrix: &mut Vec<Vec<Vec<usize>>>,
    ) {
        let mut tmp3: Vec<usize> = vec![];
        for i in 0..size {
            tmp3.push(matrix[r[0]][i][side[0]])
        }

        for i in 0..2 {
            for j in 0..size {
                matrix[r[i]][j][side[0]] = matrix[r[i + 1]][j][side[0]];
            }
        }

        let mut j = size - 1;
        for i in 0..size {
            matrix[r[2]][i][side[0]] = matrix[r[3]][j][side[1]];
            j -= 1;
        }

        j = size - 1;
        for i in 0..size {
            matrix[r[3]][i][side[1]] = tmp3[j];
            j -= 1;
        }
    }

    fn rotate_ud_layer(r: [usize; 4], side: usize, matrix: &mut Vec<Vec<Vec<usize>>>) {
        let tmp3 = matrix[r[0]][side].to_vec();
        for i in 0..3 {
            matrix[r[i]][side] = matrix[r[i + 1]][side].to_vec();
        }
        matrix[r[3]][side] = tmp3;
    }

    fn rotate_fb_layer(
        r: [usize; 4],
        size: usize,
        layer: usize,
        matrix: &mut Vec<Vec<Vec<usize>>>,
    ) {
        let tmp3 = matrix[r[0]][size - layer - 1].to_vec();

        let mut j = size - 1;
        for i in 0..size {
            matrix[r[0]][size - layer - 1][i] = matrix[r[1]][j][size - layer - 1];
            j -= 1;
        }

        for i in 0..size {
            matrix[r[1]][i][size - layer - 1] = matrix[r[2]][layer][i];
        }

        j = size - 1;
        for i in 0..size {
            matrix[r[2]][layer][i] = matrix[r[3]][j][layer];
            j -= 1;
        }

        for i in 0..size {
            matrix[r[3]][i][layer] = tmp3[i];
        }
    }

    fn rev_rotate_fb_layer(
        r: [usize; 4],
        size: usize,
        layer: usize,
        matrix: &mut Vec<Vec<Vec<usize>>>,
    ) {
        let tmp3 = matrix[r[0]][size - layer - 1].to_vec();

        for i in 0..size {
            matrix[r[0]][size - layer - 1][i] = matrix[r[1]][i][layer];
        }

        let mut j = size - 1;
        for i in 0..size {
            matrix[r[1]][i][layer] = matrix[r[2]][layer][j];
            j -= 1;
        }

        for j in 0..size {
            matrix[r[2]][layer][j] = matrix[r[3]][j][size - layer - 1];
        }

        j = size - 1;
        for i in 0..size {
            matrix[r[3]][i][size - layer - 1] = tmp3[j];
            j -= 1;
        }
    }

    fn apply_move(image: &mut Vec<Vec<Vec<usize>>>, size: usize, mv: &str) {
        match mv {
            "R" | "Rw" => {
                image[0] = CubePuzzle::rotate_2d_matrix(size, true, &image[0]);
                CubePuzzle::rotate_rl_layer([1, 2, 4, 5], size, [size - 1, 0], image);
                if mv == "Rw" {
                    CubePuzzle::rotate_rl_layer([1, 2, 4, 5], size, [size - 2, 1], image);
                }
            }
            "R2" => {
                CubePuzzle::apply_move(image, size, "R");
                CubePuzzle::apply_move(image, size, "R");
            }
            "Rw2" => {
                CubePuzzle::apply_move(image, size, "Rw");
                CubePuzzle::apply_move(image, size, "Rw");
            }
            "R'" | "Rw'" => {
                image[0] = CubePuzzle::rotate_2d_matrix(size, false, &image[0]);
                CubePuzzle::rotate_rl_layer([4, 2, 1, 5], size, [size - 1, 0], image);
                if mv == "Rw'" {
                    CubePuzzle::rotate_rl_layer([4, 2, 1, 5], size, [size - 2, 1], image);
                }
            }
            "L" | "Lw" => {
                image[3] = CubePuzzle::rotate_2d_matrix(size, true, &image[3]);
                CubePuzzle::rotate_rl_layer([4, 2, 1, 5], size, [0, size - 1], image);
                if mv == "Lw" {
                    CubePuzzle::rotate_rl_layer([4, 2, 1, 5], size, [1, size - 2], image);
                }
            }
            "L2" => {
                CubePuzzle::apply_move(image, size, "L");
                CubePuzzle::apply_move(image, size, "L");
            }
            "Lw2" => {
                CubePuzzle::apply_move(image, size, "Lw");
                CubePuzzle::apply_move(image, size, "Lw");
            }
            "L'" | "Lw'" => {
                image[3] = CubePuzzle::rotate_2d_matrix(size, false, &image[3]);
                CubePuzzle::rotate_rl_layer([1, 2, 4, 5], size, [0, size - 1], image);
                if mv == "Lw'" {
                    CubePuzzle::rotate_rl_layer([1, 2, 4, 5], size, [1, size - 2], image);
                }
            }
            "U" | "Uw" => {
                image[1] = CubePuzzle::rotate_2d_matrix(size, true, &image[1]);
                CubePuzzle::rotate_ud_layer([0, 5, 3, 2], 0, image);
                if mv == "Uw" {
                    CubePuzzle::rotate_ud_layer([0, 5, 3, 2], 1, image);
                }
            }
            "U2" => {
                CubePuzzle::apply_move(image, size, "U");
                CubePuzzle::apply_move(image, size, "U");
            }
            "Uw2" => {
                CubePuzzle::apply_move(image, size, "Uw");
                CubePuzzle::apply_move(image, size, "Uw");
            }
            "U'" | "Uw'" => {
                image[1] = CubePuzzle::rotate_2d_matrix(size, false, &image[1]);
                CubePuzzle::rotate_ud_layer([0, 2, 3, 5], 0, image);
                if mv == "Uw'" {
                    CubePuzzle::rotate_ud_layer([0, 2, 3, 5], 1, image);
                }
            }
            "D" | "Dw" => {
                image[4] = CubePuzzle::rotate_2d_matrix(size, true, &image[4]);
                CubePuzzle::rotate_ud_layer([0, 2, 3, 5], size - 1, image);
                if mv == "Dw" {
                    CubePuzzle::rotate_ud_layer([0, 2, 3, 5], size - 2, image);
                }
            }
            "D2" => {
                CubePuzzle::apply_move(image, size, "D");
                CubePuzzle::apply_move(image, size, "D");
            }
            "Dw2" => {
                CubePuzzle::apply_move(image, size, "Dw");
                CubePuzzle::apply_move(image, size, "Dw");
            }
            "D'" | "Dw'" => {
                image[4] = CubePuzzle::rotate_2d_matrix(size, false, &image[4]);
                CubePuzzle::rotate_ud_layer([0, 5, 3, 2], size - 1, image);
                if mv == "Dw'" {
                    CubePuzzle::rotate_ud_layer([0, 5, 3, 2], size - 2, image);
                }
            }
            "F" | "Fw" => {
                image[2] = CubePuzzle::rotate_2d_matrix(size, true, &image[2]);
                CubePuzzle::rotate_fb_layer([1, 3, 4, 0], size, 0, image);
                if mv == "Fw" {
                    CubePuzzle::rotate_fb_layer([1, 3, 4, 0], size, 1, image);
                }
            }
            "F2" => {
                CubePuzzle::apply_move(image, size, "F");
                CubePuzzle::apply_move(image, size, "F");
            }
            "Fw2" => {
                CubePuzzle::apply_move(image, size, "Fw");
                CubePuzzle::apply_move(image, size, "Fw");
            }
            "F'" | "Fw'" => {
                image[2] = CubePuzzle::rotate_2d_matrix(size, false, &image[2]);
                CubePuzzle::rev_rotate_fb_layer([1, 0, 4, 3], size, 0, image);
                if mv == "Fw'" {
                    CubePuzzle::rev_rotate_fb_layer([1, 0, 4, 3], size, 1, image);
                }
            }
            "B" | "Bw" => {
                image[5] = CubePuzzle::rotate_2d_matrix(size, true, &image[5]);
                CubePuzzle::rev_rotate_fb_layer([4, 3, 1, 0], size, 0, image);
                if mv == "Bw" {
                    CubePuzzle::rev_rotate_fb_layer([4, 3, 1, 0], size, 1, image);
                }
            }
            "B2" => {
                CubePuzzle::apply_move(image, size, "B");
                CubePuzzle::apply_move(image, size, "B");
            }
            "Bw2" => {
                CubePuzzle::apply_move(image, size, "Bw");
                CubePuzzle::apply_move(image, size, "Bw");
            }
            "B'" | "Bw'" => {
                image[5] = CubePuzzle::rotate_2d_matrix(size, false, &image[5]);
                CubePuzzle::rotate_fb_layer([4, 0, 1, 3], size, 0, image);
                if mv == "Bw'" {
                    CubePuzzle::rotate_fb_layer([4, 0, 1, 3], size, 1, image);
                }
            }
            _ => {
                log(&("Error, movement not recognised: ".to_string() + &*mv));
            }
        }
    }

    pub fn apply_algorithm(&mut self, scramble: &str) {
        if scramble.trim().is_empty() {
            return;
        }
        let moves = scramble.split_ascii_whitespace();
        let ap = move |i| CubePuzzle::apply_move(&mut self.image, self.size, i);

        moves.for_each(ap);
    }

    pub fn draw_scramble(&mut self, scramble: &str, color_scheme: &HashMap<&str, Color>) -> Svg {
        let mut svg: Svg = Svg {
            name: "".to_string(),
            attributes: Default::default(),
            children: vec![],
        };

        self.apply_algorithm(scramble);

        svg.init(CubePuzzle::get_preferred_size(
            self.size as i32,
            self.cubie_size,
            self.gap,
        ));

        CubePuzzle::draw_cube(
            &mut svg,
            &self.image,
            self.gap,
            self.size,
            self.cubie_size,
            color_scheme,
        );
        return svg;
    }
}
