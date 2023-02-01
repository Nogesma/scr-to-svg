use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;
use std::string::ToString;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub i32, pub i32, pub i32);

pub static RED: Color = Color(255, 0, 0);
pub static GREEN: Color = Color(0, 255, 0);
pub static BLUE: Color = Color(0, 0, 255);
pub static WHITE: Color = Color(255, 255, 255);
pub static BLACK: Color = Color(0, 0, 0);
pub static GREY: Color = Color(128, 128, 128);
pub static YELLOW: Color = Color(255, 255, 0);
pub static ORANGE: Color = Color(255, 128, 0);

lazy_static! {
    pub static ref DEFAULT_COLOR_SCHEME: HashMap<String, Color> = HashMap::from([
        ("B".to_string(), BLUE),
        ("D".to_string(), YELLOW),
        ("F".to_string(), GREEN),
        ("L".to_string(), ORANGE),
        ("R".to_string(), RED),
        ("U".to_string(), WHITE),
    ]);
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}
