use std::fmt::{Debug, Display, Formatter};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};

lazy_static! {
    pub static ref COLOR_REGEX: Regex = Regex::new(
        r"^(rgb\s*?\(\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?\))$|^(rgba\s*?\(\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(0|0\.\d*|1|1.0*)\s*?\))$|^(transparent)$|^(#([a-fA-F0-9]){3})$|^(#([a-fA-F0-9]){6}$)|(^hsl\s*?\(\s*?(000|0?\d{1,2}|[1-2]\d\d|3[0-5]\d|360)\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?\)$)|(^hsla\s*?\(\s*?(000|0?\d{1,2}|[1-2]\d\d|3[0-5]\d|360)\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(0|0\.\d*|1|1.0*)\s*?\)$)$"
    ).unwrap();
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Color {
    RGB {
        red: u8,
        green: u8,
        blue: u8,
    },
    RGBA {
        red: u8,
        green: u8,
        blue: u8,
        alpha: f64,
    },
    Transparent,
    Hex3(String),
    Hex6(String),
    HSL {
        hue: u16,
        saturation: u8,
        lightness: u8,
    },
    HSLA {
        hue: u16,
        saturation: u8,
        lightness: u8,
        alpha: f64,
    },
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::RGB { red, green, blue } => format!("rgb({}, {}, {})", red, green, blue),
            Color::RGBA { red, green, blue, alpha } => format!("rgba({}, {}, {}, {})", red, green, blue, alpha),
            Color::Transparent => String::from("transparent"),
            Color::Hex3(value) => value.to_string(),
            Color::Hex6(value) => value.to_string(),
            Color::HSL { hue, saturation, lightness } => format!("hsl({}, {}%, {}%)", hue, saturation, lightness),
            Color::HSLA { hue, saturation, lightness, alpha } => format!("hsla({}, {}%, {}%, {})", hue, saturation, lightness, alpha),
        }
    }
}

pub type ColorEntity = ValueObject<Color>;

impl Debug for ColorEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.value.to_string(), f)
    }
}

impl TryFrom<ColorEntity> for String {
    type Error = DomainError;

    fn try_from(value: ColorEntity) -> Result<Self, Self::Error> {
        Ok(value.value.to_string())
    }
}

impl From<String> for ColorEntity {
    fn from(value: String) -> Self {
        ColorEntity::new(Some(value.as_str())).unwrap()
    }
}

impl ValueObjectTrait<Color> for ColorEntity {
    fn new(value: Option<&str>) -> Result<ColorEntity, DomainError> {
        match value {
            Some(value) => {
                if COLOR_REGEX.is_match(value) {
                    match value {
                        "transparent" => Ok(ColorEntity { value: Color::Transparent }),
                        _ => {
                            let value = value.to_string();

                            if value.starts_with("#") {
                                if value.len() == 4 {
                                    Ok(ColorEntity { value: Color::Hex3(value) })
                                } else {
                                    Ok(ColorEntity { value: Color::Hex6(value) })
                                }
                            } else if value.starts_with("rgb") {
                                let rgb = value.replace("rgb", "").replace("(", "").replace(")", "").replace(" ", "");
                                let rgb: Vec<&str> = rgb.split(",").collect();

                                let red = rgb[0].parse::<u8>().unwrap();
                                let green = rgb[1].parse::<u8>().unwrap();
                                let blue = rgb[2].parse::<u8>().unwrap();

                                Ok(ColorEntity { value: Color::RGB { red, green, blue } })
                            } else if value.starts_with("rgba") {
                                let rgba = value.replace("rgba", "").replace("(", "").replace(")", "").replace(" ", "");
                                let rgba: Vec<&str> = rgba.split(",").collect();

                                let red = rgba[0].parse::<u8>().unwrap();
                                let green = rgba[1].parse::<u8>().unwrap();
                                let blue = rgba[2].parse::<u8>().unwrap();
                                let alpha = rgba[3].parse::<f64>().unwrap();

                                Ok(ColorEntity { value: Color::RGBA { red, green, blue, alpha } })
                            } else if value.starts_with("hsl") {
                                let hsl = value.replace("hsl", "").replace("(", "").replace(")", "").replace(" ", "");
                                let hsl: Vec<&str> = hsl.split(",").collect();

                                let hue = hsl[0].parse::<u16>().unwrap();
                                let saturation = hsl[1].parse::<u8>().unwrap();
                                let lightness = hsl[2].parse::<u8>().unwrap();

                                Ok(ColorEntity { value: Color::HSL { hue, saturation, lightness } })
                            } else if value.starts_with("hsla") {
                                let hsla = value.replace("hsla", "").replace("(", "").replace(")", "").replace(" ", "");
                                let hsla: Vec<&str> = hsla.split(",").collect();

                                let hue = hsla[0].parse::<u16>().unwrap();
                                let saturation = hsla[1].parse::<u8>().unwrap();
                                let lightness = hsla[2].parse::<u8>().unwrap();
                                let alpha = hsla[3].parse::<f64>().unwrap();

                                Ok(ColorEntity { value: Color::HSLA { hue, saturation, lightness, alpha } })
                            } else {
                                Err(DomainError::new("Invalid color", ""))
                            }
                        }
                    }
                } else {
                    Err(DomainError::new("Invalid color", ""))
                }
            },
            None => Err(DomainError::new("Color is required", ""))
        }
    }

    fn value(&self) -> &Color {
        &self.value
    }

    fn equals(&self, other: &ColorEntity) -> bool {
        self.value.to_string() == other.value.to_string()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}