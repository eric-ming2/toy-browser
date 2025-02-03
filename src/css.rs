#[derive(Debug, PartialEq)]
pub(crate) struct Stylesheet {
    pub(crate) rules: Vec<Rule>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Rule {
    pub(crate) selectors: Vec<Selector>,
    pub(crate) declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq)]
pub(crate) struct SimpleSelector {
    pub(crate) tag_name: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) class: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Declaration {
    pub(crate) name: String,
    pub(crate) value: Value,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color),
    // insert more values here
}

#[derive(Debug, PartialEq)]
pub(crate) enum Unit {
    Px,
    // insert more units here
}

#[derive(Debug, PartialEq)]
pub(crate) struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl TryFrom<String> for Color {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let hex = value.trim_start_matches('#'); // Remove the leading '#'
        match hex.len() {
            6 => {
                // RGB format
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                Ok(Color { r, g, b, a: 255 })
            }
            8 => {
                // RGBA format
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;
                let a = u8::from_str_radix(&hex[6..8], 16).map_err(|e| e.to_string())?;
                Ok(Color { r, g, b, a })
            }
            _ => Err("Invalid color syntax".to_string()), // Invalid format
        }
    }
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}
