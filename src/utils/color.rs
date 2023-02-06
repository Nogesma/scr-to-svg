use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: i32,
    g: i32,
    b: i32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Color { r, g, b }
    }

    pub fn red() -> Self {
        Color { r: 255, g: 0, b: 0 }
    }
    pub fn green() -> Self {
        Color { r: 0, g: 255, b: 0 }
    }
    pub fn blue() -> Self {
        Color { r: 0, g: 0, b: 255 }
    }
    pub fn white() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
    pub fn black() -> Self {
        Color { r: 0, g: 0, b: 0 }
    }
    pub fn yellow() -> Self {
        Color {
            r: 255,
            g: 255,
            b: 0,
        }
    }
    pub fn orange() -> Self {
        Color {
            r: 255,
            g: 128,
            b: 0,
        }
    }
    pub fn grey() -> Self {
        Color {
            r: 128,
            g: 128,
            b: 128,
        }
    }
}
