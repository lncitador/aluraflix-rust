use lazy_static::lazy_static;
use crate::domain::errors::domain_error::DomainError;
use crate::domain::value_objects::{ValueObject, ValueObjectTrait};

lazy_static! {
    pub static ref COLOR_REGEX: Regex = Regex::new(
        r"^(rgb\s*?\(\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?\))$|^(rgba\s*?\(\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(000|0?\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\s*?,\s*?(0|0\.\d*|1|1.0*)\s*?\))$|^(transparent)$|^(#([a-fA-F0-9]){3})$|^(#([a-fA-F0-9]){6}$)|(^hsl\s*?\(\s*?(000|0?\d{1,2}|[1-2]\d\d|3[0-5]\d|360)\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?\)$)|(^hsla\s*?\(\s*?(000|0?\d{1,2}|[1-2]\d\d|3[0-5]\d|360)\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(000|100|0?\d{2}|0?0?\d)%\s*?,\s*?(0|0\.\d*|1|1.0*)\s*?\)$)$"
    ).unwrap();
}

pub type ColorEntity = ValueObject<String>;

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