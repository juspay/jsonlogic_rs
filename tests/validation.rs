#[cfg(test)]
mod tests {
    use jsonlogic::validation::{validate, RequireAndWrapper, ValidationConfig};
    use serde_json::json;

    #[test]
    fn test_and_wrapper_validation() {
        // Require and wrapper with empty allowed
        let config = ValidationConfig {
            require_and_wrapper: Some(RequireAndWrapper { allow_empty: true }),
            ..Default::default()
        };

        let empty_logic = json!({});
        assert!(validate(&empty_logic, &config).is_ok());

        let valid_logic = json!({
            "and": [
                {"==": [{"var": "age"}, 18]},
                {">=": [{"var": "score"}, 70]}
            ]
        });
        assert!(validate(&valid_logic, &config).is_ok());

        let invalid_logic = json!({
            "==": [{"var": "age"}, 18]
        });
        assert!(validate(&invalid_logic, &config).is_err());

        // Require and wrapper with empty not allowed
        let strict_config = ValidationConfig {
            require_and_wrapper: Some(RequireAndWrapper { allow_empty: false }),
            ..Default::default()
        };


        // Invalid unwrapped logic should fail
        assert!(validate(&invalid_logic, &config).is_err());

        // Empty logic should fail
        assert!(validate(&empty_logic, &strict_config).is_err());

        // And-wrapped logic should still pass
        assert!(validate(&valid_logic, &strict_config).is_ok());
    }
}
