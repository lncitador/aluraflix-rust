#[cfg(test)]
mod test_color_value_object {
    use crate::domain::value_objects::color::{Color, ColorEntity};
    use crate::domain::value_objects::ValueObjectTrait;

    #[test]
    fn it_should_create_a_valid_color() {
        let color = ColorEntity::new(Some("rgb(255, 255, 255)"));

        match color.unwrap().value {
            Color::RGB { red, green, blue } => {
                assert_eq!(red, 255);
                assert_eq!(green, 255);
                assert_eq!(blue, 255);
            }
            _ => panic!("Invalid color")
        };
    }

    #[test]
    fn it_should_create_a_valid_color_with_hex3() {
        let color = ColorEntity::new(Some("#fff"));

        match color.unwrap().value {
            Color::Hex3(value) => {
                assert_eq!(value, "#fff");
            }
            _ => panic!("Invalid color")
        };
    }

    #[test]
    fn it_should_create_a_valid_color_with_hex6() {
        let color = ColorEntity::new(Some("#ffffff"));

        match color.unwrap().value {
            Color::Hex6(value) => {
                assert_eq!(value, "#ffffff");
            }
            _ => panic!("Invalid color")
        };
    }

    #[test]
    fn it_should_create_a_valid_color_with_rgb() {
        let color = ColorEntity::new(Some("rgb(255, 255, 255)"));

        match color.unwrap().value {
            Color::RGB { red, green, blue } => {
                assert_eq!(red, 255);
                assert_eq!(green, 255);
                assert_eq!(blue, 255);
            }
            _ => panic!("Invalid color")
        };
    }

    #[test]
    fn it_should_create_a_valid_color_with_rgba() {
        let color = ColorEntity::new(Some("rgba(255, 255, 255, 0.5)"));

        match color.unwrap().value {
            Color::RGBA { red, green, blue, alpha } => {
                assert_eq!(red, 255);
                assert_eq!(green, 255);
                assert_eq!(blue, 255);
                assert_eq!(alpha, 0.5);
            }
            _ => panic!("Invalid color")
        };
    }

    #[test]
    fn it_should_create_a_valid_color_with_hsl() {
        let hsl = "hsl(360, 100%, 100%)";
        let color = ColorEntity::new(Some(hsl)).unwrap();

        assert_eq!(color.value.to_string(), hsl);
    }

    #[test]
    fn it_should_create_a_valid_color_with_hsla() {
        let hsla = "hsla(360, 100%, 100%, 0.5)";
        let color = ColorEntity::new(Some(hsla)).unwrap();

        assert_eq!(color.value.to_string(), hsla);
    }

    #[test]
    fn it_should_not_create_a_color_with_invalid_value() {
        let color = ColorEntity::new(Some("invalid"));

        assert!(color.is_err());
    }

    #[test]
    fn it_should_not_create_a_color_with_empty_value() {
        let color = ColorEntity::new(None);

        assert!(color.is_err());
    }

    #[test]
    fn it_should_return_the_color_value() {
        let color = ColorEntity::new(Some("rgb(255, 255, 255)"));

        assert_eq!(color.unwrap().value().to_string(), "rgb(255, 255, 255)");
    }
}