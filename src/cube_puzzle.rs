use crate::color::{Color, BLACK, DEFAULT_COLOR_SCHEME};
use crate::dimension::Dimension;
use crate::element::Element;
use crate::log;
use crate::rotations::rotate_2d_matrix;
use crate::svg::Svg;
use std::collections::HashMap;

pub struct CubePuzzle {
    pub size: usize,
    pub cubie_size: usize,
    pub gap: usize,
    pub state: Vec<Vec<Vec<i8>>>,
    color_scheme: Option<HashMap<String, Color>>,
}

static FACES: [&str; 6] = ["R", "U", "F", "L", "D", "B"];

pub fn get_cube_size(event: &str) -> usize {
    match event {
        "333" | "OH" | "3BLD" => 3,
        "222" => 2,
        "444" => 4,
        "555" => 5,
        _ => 0,
    }
}

impl CubePuzzle {
    pub fn new(size: usize) -> CubePuzzle {
        let default_cube_state = (0..6)
            .map(|f| (0..size).map(|_| (0..size).map(|_| f).collect()).collect())
            .collect();

        CubePuzzle {
            size,
            cubie_size: 10,
            gap: 2,
            state: default_cube_state,
            color_scheme: None,
        }
    }

    pub fn apply_algorithm(&mut self, scramble: &str) {
        if scramble.trim().is_empty() {
            return;
        }

        let moves = scramble.split_ascii_whitespace();
        moves.for_each(|mv| self.apply_move(mv));
    }

    fn rotate_rl_layer(&mut self, r: [usize; 4], side: [usize; 2]) {
        let tmp3: Vec<i8> = self.state[r[0]].iter().map(|x| x[side[0]]).collect();

        for j in 0..self.size {
            self.state[r[0]][j][side[0]] = self.state[r[1]][j][side[0]];
        }
        for j in 0..self.size {
            self.state[r[1]][j][side[0]] = self.state[r[2]][j][side[0]];
        }

        for j in 0..self.size {
            self.state[r[2]][j][side[0]] = self.state[r[3]][self.size - j - 1][side[1]];
        }
        for j in 0..self.size {
            self.state[r[3]][j][side[1]] = tmp3[self.size - j - 1];
        }
    }

    fn rotate_ud_layer(&mut self, r: [usize; 4], side: usize) {
        let tmp3 = self.state[r[0]][side].clone();
        for i in 0..3 {
            self.state[r[i]][side] = self.state[r[i + 1]][side].clone();
        }
        self.state[r[3]][side] = tmp3;
    }

    fn rotate_fb_layer(&mut self, r: [usize; 4], layer: usize) {
        let tmp3 = self.state[r[0]][self.size - layer - 1].to_vec();

        for i in 0..self.size {
            self.state[r[0]][self.size - layer - 1][i] =
                self.state[r[1]][self.size - i - 1][self.size - layer - 1];
        }

        for i in 0..self.size {
            self.state[r[1]][i][self.size - layer - 1] = self.state[r[2]][layer][i];
        }

        for i in 0..self.size {
            self.state[r[2]][layer][i] = self.state[r[3]][self.size - i - 1][layer];
        }

        for i in 0..self.size {
            self.state[r[3]][i][layer] = tmp3[i];
        }
    }
    fn rev_rotate_fb_layer(&mut self, r: [usize; 4], layer: usize) {
        let tmp3 = self.state[r[0]][self.size - layer - 1].to_vec();

        for i in 0..self.size {
            self.state[r[0]][self.size - layer - 1][i] = self.state[r[1]][i][layer];
        }

        for i in 0..self.size {
            self.state[r[1]][i][layer] = self.state[r[2]][layer][self.size - i - 1];
        }

        for j in 0..self.size {
            self.state[r[2]][layer][j] = self.state[r[3]][j][self.size - layer - 1];
        }

        for i in 0..self.size {
            self.state[r[3]][i][self.size - layer - 1] = tmp3[self.size - i - 1];
        }
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
                rotate_2d_matrix(&mut self.state[0], self.size, true);
                self.rotate_rl_layer([1, 2, 4, 5], [self.size - 1, 0]);
                if mv == "Rw" {
                    self.rotate_rl_layer([1, 2, 4, 5], [self.size - 2, 1]);
                }
            }
            "R'" | "Rw'" => {
                rotate_2d_matrix(&mut self.state[0], self.size, false);
                self.rotate_rl_layer([4, 2, 1, 5], [self.size - 1, 0]);
                if mv == "Rw'" {
                    self.rotate_rl_layer([4, 2, 1, 5], [self.size - 2, 1]);
                }
            }
            "L" | "Lw" => {
                rotate_2d_matrix(&mut self.state[3], self.size, true);
                self.rotate_rl_layer([4, 2, 1, 5], [0, self.size - 1]);
                if mv == "Lw" {
                    self.rotate_rl_layer([4, 2, 1, 5], [1, self.size - 2]);
                }
            }
            "L'" | "Lw'" => {
                rotate_2d_matrix(&mut self.state[3], self.size, false);
                self.rotate_rl_layer([1, 2, 4, 5], [0, self.size - 1]);
                if mv == "Lw'" {
                    self.rotate_rl_layer([1, 2, 4, 5], [1, self.size - 2]);
                }
            }
            "U" | "Uw" => {
                rotate_2d_matrix(&mut self.state[1], self.size, true);
                self.rotate_ud_layer([0, 5, 3, 2], 0);
                if mv == "Uw" {
                    self.rotate_ud_layer([0, 5, 3, 2], 1);
                }
            }
            "U'" | "Uw'" => {
                rotate_2d_matrix(&mut self.state[1], self.size, false);
                self.rotate_ud_layer([0, 2, 3, 5], 0);
                if mv == "Uw'" {
                    self.rotate_ud_layer([0, 2, 3, 5], 1);
                }
            }
            "D" | "Dw" => {
                rotate_2d_matrix(&mut self.state[4], self.size, true);
                self.rotate_ud_layer([0, 2, 3, 5], self.size - 1);
                if mv == "Dw" {
                    self.rotate_ud_layer([0, 2, 3, 5], self.size - 2);
                }
            }
            "D'" | "Dw'" => {
                rotate_2d_matrix(&mut self.state[4], self.size, false);
                self.rotate_ud_layer([0, 5, 3, 2], self.size - 1);
                if mv == "Dw'" {
                    self.rotate_ud_layer([0, 5, 3, 2], self.size - 2);
                }
            }
            "F" | "Fw" => {
                rotate_2d_matrix(&mut self.state[2], self.size, true);
                self.rotate_fb_layer([1, 3, 4, 0], 0);
                if mv == "Fw" {
                    self.rotate_fb_layer([1, 3, 4, 0], 1);
                }
            }
            "F'" | "Fw'" => {
                rotate_2d_matrix(&mut self.state[2], self.size, false);
                self.rev_rotate_fb_layer([1, 0, 4, 3], 0);
                if mv == "Fw'" {
                    self.rev_rotate_fb_layer([1, 0, 4, 3], 1);
                }
            }
            "B" | "Bw" => {
                rotate_2d_matrix(&mut self.state[5], self.size, true);
                self.rev_rotate_fb_layer([4, 3, 1, 0], 0);
                if mv == "Bw" {
                    self.rev_rotate_fb_layer([4, 3, 1, 0], 1);
                }
            }
            "B'" | "Bw'" => {
                rotate_2d_matrix(&mut self.state[5], self.size, false);
                self.rotate_fb_layer([4, 0, 1, 3], 0);
                if mv == "Bw'" {
                    self.rotate_fb_layer([4, 0, 1, 3], 1);
                }
            }
            _ => {
                log(&("Error, movement not recognised: ".to_string() + mv));
            }
        }
    }

    fn get_face_index(s: &str) -> usize {
        FACES.iter().position(|&x| x == s).unwrap()
    }

    pub fn draw_cube(&self, g: &mut Svg, color_scheme: &HashMap<String, Color>) {
        let gap = self.gap;
        let size = self.size;
        let cubie_size = self.cubie_size;

        CubePuzzle::paint_cube_face(
            g,
            gap,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state[CubePuzzle::get_face_index("L")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            2 * gap + size * cubie_size,
            3 * gap + 2 * size * cubie_size,
            size,
            cubie_size,
            &self.state[CubePuzzle::get_face_index("D")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            4 * gap + 3 * size * cubie_size,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state[CubePuzzle::get_face_index("B")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            3 * gap + 2 * size * cubie_size,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state[CubePuzzle::get_face_index("R")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            2 * gap + size * cubie_size,
            gap,
            size,
            cubie_size,
            &self.state[CubePuzzle::get_face_index("U")],
            color_scheme,
        );
        CubePuzzle::paint_cube_face(
            g,
            2 * gap + size * cubie_size,
            2 * gap + size * cubie_size,
            size,
            cubie_size,
            &self.state[CubePuzzle::get_face_index("F")],
            color_scheme,
        );
    }

    pub fn paint_cube_face(
        g: &mut Svg,
        x: usize,
        y: usize,
        size: usize,
        cubie_size: usize,
        face_colors: &[Vec<i8>],
        color_scheme: &HashMap<String, Color>,
    ) {
        for row in 0..size {
            for col in 0..size {
                let tempx = x + col * cubie_size;
                let tempy = y + row * cubie_size;

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

                let col = color_scheme
                    .get(FACES[face_colors[row][col] as usize])
                    .unwrap_or(&BLACK);

                rect.set_fill(*col);
                rect.set_stroke(BLACK);
                g.append_child(rect);
            }
        }
    }

    fn get_preferred_size(&self) -> Dimension {
        let width = (self.size * self.cubie_size + self.gap) * 4 + self.gap;
        let height = (self.size * self.cubie_size + self.gap) * 3 + self.gap;

        Dimension { width, height }
    }

    pub fn draw(&self) -> Svg {
        let mut svg: Svg = Svg {
            name: "".to_string(),
            attributes: Default::default(),
            children: vec![],
        };

        svg.init(self.get_preferred_size());

        if let Some(color) = self.color_scheme.clone() {
            self.draw_cube(&mut svg, &color);
        } else {
            self.draw_cube(&mut svg, &DEFAULT_COLOR_SCHEME);
        }
        svg
    }
}
