#[cfg(test)]
mod test_url_value_object {
    use crate::domain::value_objects::url::{UrlEntity};
    use crate::domain::value_objects::ValueObjectTrait;

    const URL: &str = "https://www.google.com";

    #[test]
    fn should_create_url_value_object() {
        let url = UrlEntity::new(Some(URL)).unwrap();
        assert_eq!(url.value(), URL);
    }

    #[test]
    fn should_not_create_url_value_object() {
        let url = UrlEntity::new(Some("invalid_url"));
        assert!(url.is_err());
    }

    #[test]
    fn should_not_create_url_value_object_without_url() {
        let url = UrlEntity::new(None);
        assert!(url.is_err());
    }

    #[test]
    fn should_convert_url_value_object_to_string() {
        let url = UrlEntity::new(Some(URL)).unwrap();
        let url_string = String::try_from(url).unwrap();
        assert_eq!(url_string, URL);
    }

    #[test]
    fn should_convert_string_to_url_value_object() {
        let url = UrlEntity::from(URL.to_string());
        assert_eq!(url.value(), URL);
    }
}