#[cfg(test)]
mod test_email_value_object {
    use crate::domain::value_objects::email::{EmailEntity};
    use crate::domain::value_objects::ValueObjectTrait;

    const VALID_EMAIL: &str = "doejoe@test.com";

    #[test]
    fn it_should_create_a_email_value_object() {
        let email = EmailEntity::new(Some(VALID_EMAIL));

        assert!(email.is_ok());
    }

    #[test]
    fn it_should_not_create_a_email_value_object_with_an_invalid_email() {
        let email = EmailEntity::new(Some("invalid-email"));

        assert!(email.is_err());
    }

    #[test]
    fn it_should_not_create_a_email_value_object_with_an_empty_email() {
        let email = EmailEntity::new(Some(""));

        assert!(email.is_err());
    }
}