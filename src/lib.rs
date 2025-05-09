extern crate serde_json;

pub mod data;
pub mod expression;
pub mod operators;
pub mod validation;

use expression::Ambiguous;
use serde_json::Value;
use std::collections::HashSet;

use data::Data;

#[derive(Debug, PartialEq)]
pub enum PartialApplyOutcome {
    Resolved(Value),
    Ambiguous,
}

/// Applies the given JsonLogic rule to the specified data.
/// If the rule does not use any variables, you may pass `&Value::Null` as the second argument.
///
/// # Example
///
/// ```
/// use serde_json::{json, Value};
///
/// let rule = json!({
///     "===": [
///         2,
///         { "var": "foo" }
///     ]
/// });
///
/// let data = json!({ "foo": 2 });
/// assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(true)));
///
/// let data = json!({ "foo": 3 });
/// assert_eq!(jsonlogic::apply(&rule, &data), Ok(Value::Bool(false)));
/// ```
pub fn apply(json_logic: &Value, data: &Value) -> Result<Value, String> {
    let ast = expression::Expression::from_json(json_logic)?;
    let data = Data::from_json(data);
    Ok(ast.compute(&data))
}

pub fn partial_apply(json_logic: &Value, data: &Value) -> Result<PartialApplyOutcome, String> {
    let ast = expression::Expression::from_json(json_logic)?;
    let data = Data::from_json(data);
    let result = match ast.partial_compute(&data) {
        Ok(value) => PartialApplyOutcome::Resolved(value),
        Err(Ambiguous) => PartialApplyOutcome::Ambiguous,
    };
    Ok(result)
}

// TODO: Add to public api when ready.
#[allow(dead_code)]
fn get_variable_names(json_logic: &Value) -> Result<HashSet<String>, String> {
    let ast = expression::Expression::from_json(json_logic)?;
    ast.get_variable_names()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashSet;

    #[test]
    fn var_names() {
        let json_logic = json!({ "!==": [{ "var": "foo" }, { "var": ["bar", 5] }] });
        let names: HashSet<_> = [String::from("foo"), String::from("bar")]
            .iter()
            .cloned()
            .collect();
        assert_eq!(get_variable_names(&json_logic).unwrap(), names);
    }
}
