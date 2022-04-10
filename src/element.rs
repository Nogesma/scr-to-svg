use crate::color::Color;
use std::collections::HashMap;

pub struct Element {
    pub name: String,
    pub attributes: HashMap<String, String>,
}

impl Element {
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn get_attribute(&mut self, key: &str) {
        self.attributes.get(key);
    }

    pub fn rectangle(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.name = "rect".to_string();
        self.set_attribute("x".to_string(), x.to_string());
        self.set_attribute("y".to_string(), y.to_string());
        self.set_attribute("width".to_string(), width.to_string());
        self.set_attribute("height".to_string(), height.to_string());
    }

    pub fn set_fill(&mut self, c: Color) {
        self.set_attribute("fill".to_string(), c.to_string());
    }

    pub fn set_stroke(&mut self, c: Color) {
        self.set_attribute("stroke".to_string(), c.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut str: String = "<".to_string() + &*self.name;
        for (key, value) in &self.attributes {
            str.push(' ');
            str.push_str(&*key);
            str.push_str("=\"");
            str.push_str(&*value);
            str.push('"');
        }
        str.push_str("/>");
        return str;
    }
}
