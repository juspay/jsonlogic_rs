use serde_json::Value;

use crate::{
    expression::{self, Expression},
    operators::Operator,
};

/// Represents the result of a validation.
pub type ValidationResult = Result<(), ValidationError>;

/// Represents a validation error.
#[derive(Debug, PartialEq)]
pub struct ValidationError {
    pub message: String,
    pub path: String,
}

impl ValidationError {
    pub fn new(message: &str, path: &str) -> Self {
        ValidationError {
            message: message.to_string(),
            path: path.to_string(),
        }
    }
}

/// Configuration for validating JSON Logic expressions.
pub struct ValidationConfig {
    // Allowed operators in the JSON Logic expression.
    // pub allowed_operators: Option<HashSet<String>>,

    // Maximum depth of the JSON Logic expression tree.
    // pub max_depth: Option<usize>,

    // Required variables that must be present in the JSON Logic expression.
    // pub required_variables: Option<HashSet<String>>,

    // Allowed variables that can be accessed in the JSON Logic expression.
    // pub allowed_variables: Option<HashSet<String>>,

    // Validates the data value against a specific schema.
    // pub data_schema_validator: Option<Box<dyn Fn(&Value) -> ValidationResult>>,
    /// If Some, ensures all conditions are wrapped in an 'and' block.
    /// Empty JSON logic objects are allowed if allow_empty is true.
    pub require_and_wrapper: Option<RequireAndWrapper>,
}

/// Configuration for requiring JSON Logic to be wrapped in an 'and' block
#[derive(Debug, Clone, Copy)]
pub struct RequireAndWrapper {
    /// If true, allows empty JSON logic objects (e.g., `{}`)
    pub allow_empty: bool,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        ValidationConfig {
            // allowed_operators: None,
            // max_depth: None,
            // required_variables: None,
            // allowed_variables: None,
            // data_schema_validator: None,
            require_and_wrapper: None,
        }
    }
}

/// Validates a JSON Logic expression against the provided configuration.
pub fn validate(json_logic: &Value, config: &ValidationConfig) -> ValidationResult {
    // Check if the JSON Logic is properly wrapped in an 'and' block if required
    if let Some(require_and) = &config.require_and_wrapper {
        validate_and_wrapper(json_logic, require_and)?;
    }
    Ok(())
}

/// Validates that JSON Logic is properly wrapped in an 'and' block.
fn validate_and_wrapper(json_logic: &Value, config: &RequireAndWrapper) -> ValidationResult {
    let ast = expression::Expression::from_json(json_logic)
        .map_err(|err| ValidationError::new(&err.to_string(), "$"))?;

    match ast {
        Expression::Constant(Value::Object(obj)) if obj.is_empty() && config.allow_empty => Ok(()),
        Expression::Constant(Value::Object(obj)) if obj.is_empty() && !config.allow_empty => {
            Err(ValidationError::new("Empty JSON Logic is not allowed", "$"))
        }

        Expression::Computed(Operator::And, _) => Ok(()),
        _ => Err(ValidationError::new(
            "JSON Logic must be wrapped in an 'and' block",
            "$",
        )),
    }
}
