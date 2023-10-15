#[cfg(test)]
mod test_unique_entity_id_value_object {
    use crate::domain::value_objects::unique_id::UniqueEntityID;
    use crate::domain::value_objects::ValueObjectTrait;

    const VALID_UUID: &str = "018b33fc-e22c-79a9-9fae-2f50e95e125b";

    #[test]
    fn it_should_create_a_new_unique_entity_id() {
        let unique_entity_id = UniqueEntityID::new(None);

        assert!(unique_entity_id.is_ok());
    }

    #[test]
    fn it_should_create_a_new_unique_entity_id_with_a_valid_uuid() {
        let unique_entity_id = UniqueEntityID::new(Some(VALID_UUID));

        assert!(unique_entity_id.is_ok());
    }

    #[test]
    fn it_should_not_create_a_new_unique_entity_id_with_an_invalid_uuid() {
        let unique_entity_id = UniqueEntityID::new(Some("invalid-uuid"));

        assert!(unique_entity_id.is_err());
    }
}