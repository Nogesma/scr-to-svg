use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub i32, pub i32, pub i32);

impl Color {
    pub fn new_color(color: &str) -> Color {
        lazy_static! {
            static ref COLORS: HashMap<&'static str, Color> = HashMap::from([
                ("RED", Color(255, 0, 0)),
                ("GREEN", Color(0, 255, 0)),
                ("BLUE", Color(0, 0, 255)),
                ("WHITE", Color(255, 255, 255)),
                ("BLACK", Color(0, 0, 0)),
                ("GRAY", Color(128, 128, 128)),
                ("YELLOW", Color(255, 255, 0)),
                ("ORANGE", Color(255, 128, 0)),
            ]);
        }

        return match COLORS.get(color) {
            Some(x) => *x,
            None => Color(0, 0, 0),
        };
    }

    pub fn to_string(&self) -> String {
        return "#".to_string()
            + &*format!("{:02X}", self.0)
            + &*format!("{:02X}", self.1)
            + &*format!("{:02X}", self.2);
    }
}
