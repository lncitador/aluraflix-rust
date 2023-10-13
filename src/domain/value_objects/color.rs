use lazy_static::lazy_static;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};

lazy_static! {
    pub static ref COLOR_REGEX: Regex = Regex::new(
        r"^(rgb\s*?\(\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?\))$|^(rgba\s*?\(\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(0|0\.\d*|1|1.0*)\s*?\))$|^(transparent)$|^(#([a-fA-F0-9]){3})$|^(#([a-fA-F0-9]){6}$)|(^hsl\s*?\(\s*?(000|0?\d{1,2}|[1-2]\d\d|3[0-5]\d|360)\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?\)$)|(^hsla\s*?\(\s*?(000|0?\d{1,2}|[1-2]\d\d|3[0-5]\d|360)\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(0|0\.\d*|1|1.0*)\s*?\)$)$"
    ).unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
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

impl ValueObjectTrait<String> for ColorEntity {
    fn new(value: Option<&str>) -> Result<ColorEntity, DomainError> {
        match value {
            Some(value) => {
                if COLOR_REGEX.is_match(value) {
                    Ok(ColorEntity {
                        value: value.to_string()
                    })
                } else {
                    Err(DomainError::new("Invalid color", ""))
                }
            },
            None => Err(DomainError::new("Color is required", ""))
        }
    }

    fn value(&self) -> &String {
        &self.value
    }

    fn equals(&self, other: &ColorEntity) -> bool {
        &self.value == other.value()
    }

    fn to_string(&self) -> String {
        self.value.to_string()
    }
}