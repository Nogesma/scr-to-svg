use crate::puzzles::{ColorSchemes, Puzzle};
use crate::utils::color::Color;
use crate::utils::maths::get_line_intersection;
use lazy_static::lazy_static;
use ndarray::{s, Array2, ArrayView1};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::fmt;
use svg::node::element::path::Position::Absolute;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::{Path, SVG};
use svg::{Document, Node};

#[derive(Debug)]
pub struct Megaminx {
    pub state: Array2<Face>,
    color_scheme: HashMap<Face, Color>,
    minx_rad: f64,
    gap: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Face {
    U,
    Bl,
    Br,
    R,
    F,
    L,
    D,
    Dr,
    Dbr,
    B,
    Dbl,
    Dl,
}

impl From<usize> for Face {
    fn from(x: usize) -> Self {
        match x {
            0 => Face::U,
            1 => Face::Bl,
            2 => Face::Br,
            3 => Face::R,
            4 => Face::F,
            5 => Face::L,
            6 => Face::D,
            7 => Face::Dr,
            8 => Face::Dbr,
            9 => Face::B,
            10 => Face::Dbl,
            11 => Face::Dl,
            _ => panic!(),
        }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

lazy_static! {
    pub static ref DEFAULT_COLOR_SCHEME: HashMap<Face, Color> = HashMap::from([
        (Face::B, Color::new(0x71, 0xe6, 0)),
        (Face::D, Color::new(0x99, 0x99, 0x99)),
        (Face::F, Color::new(0, 0x66, 0)),
        (Face::L, Color::new(0x8a, 0x1a, 0xff)),
        (Face::R, Color::new(0xdd, 0, 0)),
        (Face::U, Color::white()),
        (Face::Bl, Color::new(0xff, 0xcc, 0)),
        (Face::Br, Color::new(0, 0, 0xb3)),
        (Face::Dr, Color::new(0xff, 0xff, 0xb3)),
        (Face::Dbr, Color::new(0xff, 0x99, 0xff)),
        (Face::Dbl, Color::new(0xff, 0x84, 0x33)),
        (Face::Dl, Color::new(0x88, 0xdd, 0xff))
    ]);
    pub static ref UNFOLDHEIGHT: f64 = 2. + 3. * (0.3 * PI).sin() + (0.1 * PI).sin();
    pub static ref UNFOLDWIDTH: f64 = 4. * (0.1 * PI).cos() + 2. * (0.3 * PI).cos();
}

impl Puzzle for Megaminx {
    fn new() -> Self {
        Megaminx {
            gap: 2.,
            minx_rad: 30.,
            state: Array2::<Face>::from_shape_fn((12, 11), |(i, _)| Face::from(i)),
            color_scheme: DEFAULT_COLOR_SCHEME.clone(),
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
        ColorSchemes::Megaminx(DEFAULT_COLOR_SCHEME.clone())
    }

    fn draw(&self) -> SVG {
        let (width, height) = self.get_preferred_size();

        let mut svg = Document::new()
            .set("viewBox", (0, 0, width, height))
            .set("height", "100%")
            .set("width", "100%");

        self.draw_minx(&mut svg);
        svg
    }
}

impl Megaminx {
    fn pentagon(&self, xt: f64, yt: f64, pointup: bool) -> Data {
        const LEN: usize = 5;

        let mut angs = [1.3, 1.7, 0.1, 0.5, 0.9];

        for ang in angs.iter_mut() {
            if pointup {
                *ang -= 0.2;
            }
            *ang *= PI;
        }

        let mut x = [0.; LEN];
        let mut y = [0.; LEN];

        for i in 0..LEN {
            x[i] = self.minx_rad * angs[i].cos();
            y[i] = self.minx_rad * angs[i].sin();
        }

        Data::new()
            .move_to((x[0] + xt, y[0] + yt))
            .line_to((x[1] + xt, y[1] + yt))
            .line_to((x[2] + xt, y[2] + yt))
            .line_to((x[3] + xt, y[3] + yt))
            .line_to((x[4] + xt, y[4] + yt))
            .close()
    }

    fn get_face_boundaries(&self) -> HashMap<Face, Data> {
        let x = self.minx_rad * (2. * (1. - (0.6 * PI).cos())).sqrt();
        let a = self.minx_rad * (0.1 * PI).cos();
        let b = x * (0.1 * PI).cos();
        let c = x * (0.3 * PI).cos();
        let d = x * (0.1 * PI).sin();
        let e = x * (0.3 * PI).sin();

        let left_center_x = self.gap + a + b + d / 2.;
        let left_center_y = self.gap + x + self.minx_rad - d;

        let f = (0.1 * PI).cos();
        let gg = (0.2 * PI).cos();

        let magic_shift_number = d * 0.6 + self.minx_rad * (f + gg);
        let shift = left_center_x + magic_shift_number;

        let mut faces = HashMap::new();
        faces.insert(Face::U, self.pentagon(left_center_x, left_center_y, true));
        faces.insert(
            Face::Bl,
            self.pentagon(left_center_x - c, left_center_y - e, false),
        );
        faces.insert(
            Face::Br,
            self.pentagon(left_center_x + c, left_center_y - e, false),
        );
        faces.insert(
            Face::R,
            self.pentagon(left_center_x + b, left_center_y + d, false),
        );
        faces.insert(
            Face::F,
            self.pentagon(left_center_x, left_center_y + x, false),
        );
        faces.insert(
            Face::L,
            self.pentagon(left_center_x - b, left_center_y + d, false),
        );
        faces.insert(
            Face::D,
            self.pentagon(
                shift + self.gap + a + b,
                self.gap + x + self.minx_rad,
                false,
            ),
        );
        faces.insert(
            Face::Dr,
            self.pentagon(
                shift + self.gap + a + b - c,
                self.gap + x + e + self.minx_rad,
                true,
            ),
        );
        faces.insert(
            Face::Dbr,
            self.pentagon(shift + self.gap + a, self.gap + x - d + self.minx_rad, true),
        );
        faces.insert(
            Face::B,
            self.pentagon(shift + self.gap + a + b, self.gap + self.minx_rad, true),
        );
        faces.insert(
            Face::Dbl,
            self.pentagon(
                shift + self.gap + a + 2. * b,
                self.gap + x - d + self.minx_rad,
                true,
            ),
        );
        faces.insert(
            Face::Dl,
            self.pentagon(
                shift + self.gap + a + b + c,
                self.gap + x + e + self.minx_rad,
                true,
            ),
        );
        faces
    }

    fn draw_minx(&self, g: &mut SVG) {
        let pentagons = self.get_face_boundaries();

        for face in pentagons.keys() {
            let f = *face as usize;

            let rotate_counter_clockwise = match f {
                0 => 0,
                x if (1..=5).contains(&x) => 1,
                x if (6..=11).contains(&x) => 2,
                _ => panic!(),
            };

            let label = if face == &Face::U || face == &Face::F {
                Some(face.to_string())
            } else {
                None
            };

            Megaminx::draw_pentagon(
                g,
                pentagons.get(face).unwrap(),
                &self.state.slice(s![f, ..]),
                rotate_counter_clockwise,
                label,
                &self.color_scheme,
            );
        }
    }

    fn draw_pentagon(
        g: &mut SVG,
        p: &Data,
        state: &ArrayView1<Face>,
        rotate_counter_clockwise: usize,
        label: Option<String>,
        color_scheme: &HashMap<Face, Color>,
    ) {
        const LEN: usize = 5;

        let mut xpoints = [0_f64; LEN];
        let mut ypoints = [0_f64; LEN];

        for i in 0..5 {
            match p.get(i).unwrap() {
                Command::Move(_, params) | Command::Line(_, params) => {
                    xpoints[i] = *params.first().unwrap() as f64;
                    ypoints[i] = *params.last().unwrap() as f64;
                }
                _ => {}
            }
        }

        let mut xs = [0_f64; LEN * 2];
        let mut ys = [0_f64; LEN * 2];

        for i in 0..5 {
            xs[i] = 0.4 * xpoints[(i + 1) % 5] + 0.6 * xpoints[i];
            ys[i] = 0.4 * ypoints[(i + 1) % 5] + 0.6 * ypoints[i];
            xs[i + 5] = 0.6 * xpoints[(i + 1) % 5] + 0.4 * xpoints[i];
            ys[i + 5] = 0.6 * ypoints[(i + 1) % 5] + 0.4 * ypoints[i];
        }

        let mut ps = Vec::with_capacity(11);

        for _ in 0..11 {
            ps.push(Data::new())
        }

        let mut intpent = [(0., 0.); LEN];

        for i in 0..LEN {
            intpent[i] = get_line_intersection(
                xs[i],
                ys[i],
                xs[5 + (3 + i) % 5],
                ys[5 + (3 + i) % 5],
                xs[(i + 1) % 5],
                ys[(i + 1) % 5],
                xs[5 + (4 + i) % 5],
                ys[5 + (4 + i) % 5],
            );
            if i == 0 {
                ps[10].append(Command::Move(
                    Absolute,
                    Parameters::from((intpent[i].0, intpent[i].1)),
                ));
            } else {
                ps[10].append(Command::Line(
                    Absolute,
                    Parameters::from((intpent[i].0, intpent[i].1)),
                ));
            }
        }
        ps[10].append(Command::Close);

        for i in 0..LEN {
            ps[2 * i].append(Command::Move(
                Absolute,
                Parameters::from((xpoints[i], ypoints[i])),
            ));
            ps[2 * i].append(Command::Line(Absolute, Parameters::from((xs[i], ys[i]))));
            ps[2 * i].append(Command::Line(
                Absolute,
                Parameters::from((intpent[i].0, intpent[i].1)),
            ));
            ps[2 * i].append(Command::Line(
                Absolute,
                Parameters::from((xs[5 + (4 + i) % 5], ys[5 + (4 + i) % 5])),
            ));
            ps[2 * i].append(Command::Close);

            ps[2 * i + 1].append(Command::Move(Absolute, Parameters::from((xs[i], ys[i]))));
            ps[2 * i + 1].append(Command::Line(
                Absolute,
                Parameters::from((xs[i + 5], ys[i + 5])),
            ));
            ps[2 * i + 1].append(Command::Line(
                Absolute,
                Parameters::from((intpent[(i + 1) % 5].0, intpent[(i + 1) % 5].1)),
            ));
            ps[2 * i + 1].append(Command::Line(
                Absolute,
                Parameters::from((intpent[i].0, intpent[i].1)),
            ));
            ps[2 * i + 1].append(Command::Close);
        }

        for (i, p) in ps.iter().enumerate() {
            let j = if i < 10 {
                (i + 2 * rotate_counter_clockwise) % 10
            } else {
                i
            };
            let path = Path::new()
                .set("fill", color_scheme.get(&state[j]).unwrap().to_string())
                .set("stroke", "black")
                .set("d", p.clone());
            g.append(path);
        }

        if let Some(label) = label {
            let mut center_x = 0.;
            let mut center_y = 0.;
            for pt in intpent {
                center_x += pt.0;
                center_y += pt.1;
            }

            center_x /= intpent.len() as f64;
            center_y /= intpent.len() as f64;

            let mut label_text = svg::node::element::Text::new();

            label_text.assign("x", center_x);
            label_text.assign("y", center_y);
            label_text.assign("text-anchor", "middle");
            label_text.assign("dy", "0.7ex");
            label_text.append(svg::node::Text::new(label));

            //label_text.setStyle("dominant-baseline", "central");
            g.append(label_text);
        }
    }

    fn get_preferred_size(&self) -> (f64, f64) {
        (
            *UNFOLDWIDTH * 2. * self.minx_rad + 3. * self.gap,
            *UNFOLDHEIGHT * self.minx_rad + 2. * self.gap,
        )
    }
    fn apply_move(&mut self, mv: &str) {
        match mv {
            "R++" => self.big_turn(Face::Dbr, 2),
            "R--" => self.big_turn(Face::Dbr, 3),
            "D++" => self.big_turn(Face::D, 2),
            "D--" => self.big_turn(Face::D, 3),
            "U" => self.turn(Face::U, 1),
            "U'" => self.turn(Face::U, 4),
            _ => panic!(),
        }
    }

    fn turn(&mut self, f: Face, dir: i32) {
        let dir = dir.rem_euclid(5);
        for _ in 0..dir {
            self._turn(f);
        }
    }

    fn _turn(&mut self, f: Face) {
        let s = f as usize;
        let b = if s >= 6 { 6 } else { 0 };

        match s % 6 {
            0 => self.swap_on_side(b, 1, 6, 5, 4, 4, 2, 3, 0, 2, 8),
            1 => self.swap_on_side(b, 0, 0, 2, 0, 9, 6, 10, 6, 5, 2),
            2 => self.swap_on_side(b, 0, 2, 3, 2, 8, 4, 9, 4, 1, 4),
            3 => self.swap_on_side(b, 0, 4, 4, 4, 7, 2, 8, 2, 2, 6),
            4 => self.swap_on_side(b, 0, 6, 5, 6, 11, 0, 7, 0, 3, 8),
            5 => self.swap_on_side(b, 0, 8, 1, 8, 10, 8, 11, 8, 4, 0),
            _ => panic!(),
        }

        self.rotate_face(f);
    }

    fn swap_on_side(
        &mut self,
        b: usize,
        f1: usize,
        s1: usize,
        f2: usize,
        s2: usize,
        f3: usize,
        s3: usize,
        f4: usize,
        s4: usize,
        f5: usize,
        s5: usize,
    ) {
        for i in 0..3 {
            let temp = self.state[[(f1 + b) % 12, (s1 + i) % 10]];
            self.state[[(f1 + b) % 12, (s1 + i) % 10]] = self.state[[(f2 + b) % 12, (s2 + i) % 10]];
            self.state[[(f2 + b) % 12, (s2 + i) % 10]] = self.state[[(f3 + b) % 12, (s3 + i) % 10]];
            self.state[[(f3 + b) % 12, (s3 + i) % 10]] = self.state[[(f4 + b) % 12, (s4 + i) % 10]];
            self.state[[(f4 + b) % 12, (s4 + i) % 10]] = self.state[[(f5 + b) % 12, (s5 + i) % 10]];
            self.state[[(f5 + b) % 12, (s5 + i) % 10]] = temp;
        }
    }

    fn big_turn(&mut self, f: Face, dir: i32) {
        let dir = dir.rem_euclid(5);

        for _ in 0..dir {
            self._big_turn(f);
        }
    }

    fn _big_turn(&mut self, f: Face) {
        if f == Face::Dbr {
            for i in 0..7 {
                self.swap(
                    0,
                    (1 + i) % 10,
                    4,
                    (3 + i) % 10,
                    11,
                    (1 + i) % 10,
                    10,
                    (1 + i) % 10,
                    1,
                    (1 + i) % 10,
                );
            }

            self.swap_centers(0, 4, 11, 10, 1);

            self.swap_whole_face(2, 0, 3, 0, 7, 0, 6, 8, 9, 8);

            self.rotate_face(Face::Dbr);
        } else {
            assert_eq!(f, Face::D);
            for i in 0..7 {
                self.swap(
                    1,
                    (9 + i) % 10,
                    2,
                    (1 + i) % 10,
                    3,
                    (3 + i) % 10,
                    4,
                    (5 + i) % 10,
                    5,
                    (7 + i) % 10,
                );
            }

            self.swap_centers(1, 2, 3, 4, 5);

            self.swap_whole_face(11, 0, 10, 8, 9, 6, 8, 4, 7, 2);

            self.rotate_face(Face::D);
        }
    }

    fn swap(
        &mut self,
        f1: usize,
        s1: usize,
        f2: usize,
        s2: usize,
        f3: usize,
        s3: usize,
        f4: usize,
        s4: usize,
        f5: usize,
        s5: usize,
    ) {
        let temp: Face = self.state[[f1, s1]];
        self.state[[f1, s1]] = self.state[[f2, s2]];
        self.state[[f2, s2]] = self.state[[f3, s3]];
        self.state[[f3, s3]] = self.state[[f4, s4]];
        self.state[[f4, s4]] = self.state[[f5, s5]];
        self.state[[f5, s5]] = temp;
    }

    fn swap_centers(&mut self, f1: usize, f2: usize, f3: usize, f4: usize, f5: usize) {
        self.swap(f1, 10, f2, 10, f3, 10, f4, 10, f5, 10);
    }

    fn swap_whole_face(
        &mut self,
        f1: usize,
        s1: usize,
        f2: usize,
        s2: usize,
        f3: usize,
        s3: usize,
        f4: usize,
        s4: usize,
        f5: usize,
        s5: usize,
    ) {
        for i in 0..10 {
            let temp: Face = self.state[[(f1) % 12, (s1 + i) % 10]];
            self.state[[(f1) % 12, (s1 + i) % 10]] = self.state[[(f2) % 12, (s2 + i) % 10]];
            self.state[[(f2) % 12, (s2 + i) % 10]] = self.state[[(f3) % 12, (s3 + i) % 10]];
            self.state[[(f3) % 12, (s3 + i) % 10]] = self.state[[(f4) % 12, (s4 + i) % 10]];
            self.state[[(f4) % 12, (s4 + i) % 10]] = self.state[[(f5) % 12, (s5 + i) % 10]];
            self.state[[(f5) % 12, (s5 + i) % 10]] = temp;
        }

        self.swap_centers(f1, f2, f3, f4, f5);
    }

    fn rotate_face(&mut self, f: Face) {
        self.swap_on_face(f, 0, 8, 6, 4, 2);
        self.swap_on_face(f, 1, 9, 7, 5, 3);
    }

    fn swap_on_face(&mut self, face: Face, s1: usize, s2: usize, s3: usize, s4: usize, s5: usize) {
        let f = face as usize;
        let temp = self.state[[f, s1]];
        self.state[[f, s1]] = self.state[[f, s2]];
        self.state[[f, s2]] = self.state[[f, s3]];
        self.state[[f, s3]] = self.state[[f, s4]];
        self.state[[f, s4]] = self.state[[f, s5]];
        self.state[[f, s5]] = temp;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    use Face::*;
    #[test]
    fn megaminx() {
        {
            let mut mega = Megaminx::new();

            mega.apply_scramble(
                "R-- D++ R++ D-- R-- D++ R-- D-- R++ D-- U' \
                         R++ D-- R++ D++ R-- D++ R++ D-- R++ D++ U \
                         R-- D-- R-- D++ R-- D++ R++ D++ R++ D-- U' \
                         R++ D-- R++ D++ R++ D++ R++ D++ R++ D++ U \
                         R-- D-- R++ D-- R-- D++ R-- D-- R-- D-- U' \
                         R-- D-- R-- D-- R++ D-- R-- D++ R++ D++ U \
                         R++ D-- R-- D++ R++ D-- R++ D-- R++ D++ U",
            );

            assert_eq!(
                mega.state,
                arr2(&[
                    [Br, F, U, Dbr, Dl, Br, Bl, Dr, Dbr, Dbr, Br],
                    [U, B, Dbl, L, Dr, U, L, U, B, Dr, B],
                    [Bl, R, D, Dl, R, Dr, R, Br, F, B, Dbr],
                    [Br, U, Dr, Bl, Br, B, B, Br, Br, Bl, R],
                    [L, Br, Dr, Dl, U, U, Dbr, L, Dl, D, U],
                    [Dr, Dbl, F, Dbr, Bl, D, R, B, Dl, L, Bl],
                    [B, Dl, Dl, Dbl, F, F, L, Bl, Dbr, B, Dl],
                    [D, Dbl, B, Dbr, Dbl, Dbr, Dbl, F, Dbr, L, F],
                    [L, R, Bl, D, U, R, U, R, R, Dl, Dr],
                    [Dbl, R, Dr, Br, F, L, Dbr, F, B, Dr, D],
                    [Dl, F, R, Dbl, Bl, D, D, Dbl, L, D, Dbl],
                    [F, Dl, Dbl, Bl, D, Bl, Br, Dr, D, U, L]
                ])
            );
        }
        {
            let mut mega = Megaminx::new();

            mega.apply_scramble(
                "
R++ D-- R++ D-- R-- D++ R++ D-- R-- D-- U'
R++ D-- R-- D-- R++ D++ R-- D-- R++ D++ U
R++ D-- R-- D-- R++ D-- R-- D-- R-- D++ U
R-- D++ R-- D-- R++ D++ R++ D-- R++ D++ U
R++ D-- R-- D-- R-- D++ R-- D-- R-- D-- U'
R++ D++ R++ D-- R++ D-- R-- D++ R-- D-- U'
R++ D++ R-- D-- R-- D++ R-- D-- R-- D-- U'",
            );

            assert_eq!(
                mega.state,
                arr2(&[
                    [F, D, Bl, D, Dr, Br, Dbr, Br, B, Br, R],
                    [Dr, Br, Dbr, D, B, R, Dbl, Dbr, R, Dbl, Dr],
                    [B, U, Bl, Dbl, L, F, Dbr, F, F, Dbl, F],
                    [D, R, R, Dl, B, B, Dbr, Dbr, Dr, D, U],
                    [Bl, B, Dbl, Dbr, Dr, Dl, Dl, L, Dbl, Br, Br],
                    [D, F, Dl, B, U, U, D, Dbr, U, Bl, Dbr],
                    [U, Dbr, Dbl, R, Dl, F, U, Dr, U, Bl, Dbl],
                    [Bl, Dbl, D, R, R, Bl, Dl, Dr, R, F, Bl],
                    [D, B, Dr, D, Br, L, Dl, Dbl, Dbl, Dr, L],
                    [F, U, L, Bl, F, Dl, Br, Dr, Br, L, Dl],
                    [B, U, L, Dl, L, Dl, R, B, F, Dr, D],
                    [Br, Bl, L, R, Br, L, Bl, L, Dbr, U, B]
                ])
            );
        }
        {
            let mut mega = Megaminx::new();

            mega.apply_scramble(
                "
R-- D++ R++ D++ R++ D++ R-- D++ R++ D-- U'
R++ D-- R-- D++ R++ D-- R++ D++ R++ D-- U'
R++ D-- R-- D-- R++ D++ R-- D-- R-- D++ U
R-- D-- R++ D-- R++ D++ R++ D-- R-- D-- U'
R++ D-- R++ D++ R++ D++ R++ D-- R++ D-- U'
R++ D-- R-- D++ R++ D-- R-- D++ R++ D-- U'
R-- D++ R++ D++ R-- D++ R-- D-- R-- D++ U
",
            );

            assert_eq!(
                mega.state,
                arr2(&[
                    [F, R, D, Dbl, Bl, L, Dbr, U, Dr, Dl, Dbr],
                    [Br, Bl, D, L, B, U, Dl, F, R, D, R],
                    [Dr, R, Br, Dbr, Bl, Br, D, L, Dbl, B, Br],
                    [B, Dbl, B, Dl, Dbr, Br, Dl, F, D, Dr, B],
                    [Dl, Dbr, R, L, D, Dl, Dbl, Dl, Dbl, Br, D],
                    [L, L, R, Dbr, U, Dbl, Dbr, Dr, Dr, Bl, Dr],
                    [U, Dl, Bl, F, R, Br, L, B, Br, B, L],
                    [Bl, D, B, Dr, L, D, U, F, Br, Dbl, Dbl],
                    [L, Dr, Dbl, B, Dr, Dbr, Br, Bl, Dr, U, Bl],
                    [F, R, F, Dbl, B, B, Dbr, F, Dl, Dr, U],
                    [F, R, Dbr, Bl, Dl, R, Dbl, U, U, Bl, F],
                    [F, U, L, D, Bl, D, R, Dbr, U, Br, Dl]
                ])
            );
        }
        {
            let mut mega = Megaminx::new();

            mega.apply_scramble(
                "
R-- D++ R-- D-- R++ D-- R-- D++ R-- D-- U'
R-- D-- R++ D++ R-- D-- R-- D-- R++ D++ U
R++ D++ R-- D-- R-- D-- R++ D++ R-- D-- U'
R-- D-- R++ D++ R++ D++ R++ D++ R-- D-- U'
R++ D++ R++ D++ R++ D++ R-- D++ R-- D-- U'
R++ D-- R++ D++ R++ D-- R-- D-- R-- D++ U
R++ D-- R-- D++ R-- D++ R-- D-- R-- D++ U
",
            );

            assert_eq!(
                mega.state,
                arr2(&[
                    [Br, F, L, L, B, U, D, R, Br, U, U],
                    [Dbl, U, Dbl, Br, F, Dbr, F, Dl, Bl, U, Br],
                    [Dl, D, Dr, Bl, Dbl, F, Dr, Dr, Bl, Dl, R],
                    [Dbr, R, Dbl, Dbr, Dbr, Dbr, Dbr, U, Bl, Dbl, F],
                    [Dl, Bl, R, Br, B, Dl, L, Dr, Br, Br, L],
                    [R, Dr, Dl, F, U, L, U, B, Dr, D, Bl],
                    [Bl, L, R, Dbl, F, D, Dl, B, Bl, Dr, D],
                    [B, F, U, Br, Dr, Dbl, Br, Dbl, Dbl, F, Dl],
                    [Dbr, B, D, R, R, L, Dl, Bl, L, D, Dr],
                    [Dr, Dr, U, L, L, Dbl, R, Bl, B, Br, Dbr],
                    [U, D, B, Dbr, D, B, D, Bl, D, Dl, B],
                    [F, Dbr, Dbr, R, L, R, Br, Dl, F, B, Dbl]
                ])
            );
        }
        {
            let mut mega = Megaminx::new();

            mega.apply_scramble(
                "
R++ D-- R++ D-- R-- D-- R-- D-- R++ D++ U
R-- D++ R-- D++ R++ D++ R++ D++ R++ D++ U
R-- D-- R-- D++ R-- D++ R++ D++ R++ D-- U'
R-- D-- R-- D-- R++ D-- R++ D-- R-- D-- U'
R++ D-- R++ D-- R-- D-- R-- D-- R-- D++ U
R++ D++ R++ D++ R++ D-- R-- D++ R-- D-- U'
R-- D-- R++ D-- R-- D-- R++ D++ R++ D++ U
",
            );

            assert_eq!(
                mega.state,
                arr2(&[
                    [D, Bl, L, Br, B, Bl, Dbr, B, Br, F, Bl],
                    [F, Dbl, R, Dbr, Dr, D, U, Dbl, Dbl, Bl, Dbl],
                    [F, Dl, Dl, L, D, Br, U, Bl, Bl, U, B],
                    [D, U, Dbl, B, Bl, U, Bl, L, Dl, D, Br],
                    [Dl, Dr, B, Dbr, Dr, Dbl, F, F, F, F, U],
                    [R, Br, R, Br, B, L, Dbr, R, Dbl, Dr, L],
                    [L, L, Bl, Dr, U, L, Br, B, R, Dbr, Dr],
                    [Dr, Dl, L, Bl, Br, Dr, L, Dl, Dbl, Dbr, R],
                    [Dbl, D, B, R, L, B, B, U, Br, Dl, Dbr],
                    [U, Dbl, R, F, Dbr, U, D, D, Dr, B, D],
                    [U, Dbr, Br, D, Bl, Dbl, Dbr, Dl, Dr, R, Dl],
                    [D, R, Dl, Dr, Dl, Br, Dbr, R, F, F, F]
                ])
            );
        }
    }
}
