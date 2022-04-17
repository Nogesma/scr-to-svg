use crate::dimension::Dimension;
use crate::element::Element;
use std::collections::HashMap;

pub struct Svg {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<Element>,
}

impl Svg {
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn get_attribute(&mut self, key: &str) {
        self.attributes.get(key);
    }

    pub fn append_child(&mut self, value: Element) {
        self.children.push(value);
    }

    pub fn set_size(&mut self, size: Dimension) {
        self.set_attribute("width".to_string(), "100%".to_string());
        self.set_attribute("height".to_string(), "100%".to_string());
        self.set_attribute(
            "viewBox".to_string(),
            "0 0 ".to_string()
                + &size.width.to_string()
                + &" ".to_string()
                + &size.height.to_string(),
        );
    }

    pub fn init(&mut self, size: Dimension) {
        self.name = "svg".to_string();
        self.set_size(size);
        self.set_attribute("version".to_string(), "1.1".to_string());
        self.set_attribute(
            "xmlns".to_string(),
            "http://www.w3.org/2000/svg".to_string(),
        );
    }

    // fn attribute_to_string() -> String {}

    pub fn to_string(&self) -> String {
        let mut str: String = "<".to_string() + &*self.name;
        for (key, value) in &self.attributes {
            str.push(' ');
            str.push_str(&*key);
            str.push_str("=\"");
            str.push_str(&*value);
            str.push('"');
        }
        str.push('>');

        str.push_str("<g transform=\"matrix(1.0,0.0,0.0,1.0,0.5,0.5)\">");

        for child in &self.children {
            str.push_str(&*child.to_string());
        }

        str.push_str("</g>");

        str.push_str("</");
        str.push_str(&*self.name);
        str.push('>');
        return str;
    }
}
