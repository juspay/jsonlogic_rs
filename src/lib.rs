extern crate serde_json;

use serde_json::Value;

mod operators;

enum Operator {
    /// Tests equality, with type coercion. Requires two arguments.
    Equality,
    /// Tests strict equality. Requires two arguments.
    StrictEquality,
    /// Tests not-equal, with type coercion.
    NotEqual,
    /// Tests strict not-equal.
    StrictNotEqual,
}

impl Operator {
    /// Returns the Operator matching the given string representation. Returns None if the given
    /// string matches no known operator.
    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "==" => Some(Operator::Equality),
            "===" => Some(Operator::StrictEquality),
            "!=" => Some(Operator::NotEqual),
            "!==" => Some(Operator::StrictNotEqual),
            _ => None,
        }
    }
}

fn compute_double_negation(argument: &Value) -> Result<Value, String> {
    unimplemented!();
}

pub fn apply(json: &serde_json::Value) -> Result<serde_json::Value, String> {
    if !json.is_object() {
        // Return simple values.
        // TODO: Avoid cloning if possible.
        return Ok(json.clone());
    }

    let object = match json.as_object() {
        Some(v) => v,
        None => unreachable!(),
    };

    // If this object has more than one key-value pair, we will return it as is. This replicates the
    // behaviour of the javascript implementation.
    if object.len() != 1 {
        // TODO: Avoid cloning if possible.
        return Ok(json.clone());
    }

    let entry: Vec<(&String, &serde_json::Value)> = object.iter().collect();

    let &(operator_key, value) = match entry.get(0) {
        Some(v) => v,
        None => unreachable!(),
    };
    let operator = match Operator::from_str(operator_key) {
        Some(o) => o,
        None => return Err(format!("Unrecognized operation {}", operator_key)),
    };

    // TODO: To allow nested expressions, process all values here and pass them as an array to the
    // operators as an vector.

    match operator {
        Operator::Equality => operators::compute_equality(value),
        Operator::NotEqual => operators::compute_not_equal(value),
        Operator::StrictEquality => operators::compute_strict_equality(value),
        Operator::StrictNotEqual => operators::compute_strict_not_equal(value),
        _ => panic!("Not implemented"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn truthy_values() {
        // See http://jsonlogic.com/truthy.html
        assert_eq!(operators::truthy(&json!(0)), false);
        assert_eq!(operators::truthy(&json!(-1)), true);
        assert_eq!(operators::truthy(&json!(1)), true);
        assert_eq!(operators::truthy(&json!([])), false);
        assert_eq!(operators::truthy(&json!([1, 2])), true);
        assert_eq!(operators::truthy(&json!("")), false);
        assert_eq!(operators::truthy(&json!("anything")), true);
        assert_eq!(operators::truthy(&json!("0")), true);
        assert_eq!(operators::truthy(&Value::Null), false);

        assert_eq!(operators::truthy(&json!({})), true);
        assert_eq!(operators::truthy(&json!(true)), true);
        assert_eq!(operators::truthy(&json!(false)), false);
    }

    #[test]
    fn simple_values() {
        let num = json!(1);
        assert_eq!(apply(&num), Ok(num));

        let string = json!("foo");
        assert_eq!(apply(&string), Ok(string));

        let boolean = json!(true);
        assert_eq!(apply(&boolean), Ok(boolean));
    }

    // ==
    mod equality {
        use super::*;

        fn test_equality(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "==": [a, b] })), Ok(Value::Bool(expect)));
            assert_eq!(apply(&json!({ "==": [b, a] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_equality() {
            assert_eq!(apply(&json!({ "==": [] })), Ok(Value::Bool(true)));
            // For whatever reason the javascript implementation returns true for `null` instead of an
            // array as the argument.
            assert_eq!(apply(&json!({ "==": Value::Null })), Ok(Value::Bool(true)));
            assert_eq!(
                apply(&json!({ "==": [Value::Null] })),
                Ok(Value::Bool(true))
            );
            test_equality(&json!(null), &json!(null), true);
            test_equality(&json!(1), &json!(1), true);
            test_equality(&json!("foo"), &json!("foo"), true);

            assert_eq!(apply(&json!({ "==": 0 })), Ok(Value::Bool(false)));
            assert_eq!(apply(&json!({ "==": [0] })), Ok(Value::Bool(false)));
            test_equality(&json!(1), &json!(0), false);
        }

        #[test]
        fn equality_with_type_coercion() {
            test_equality(&json!(0), &json!(""), true);
            test_equality(&json!(1), &json!("1"), true);
            test_equality(&json!([1]), &json!("1"), true);
            test_equality(&json!([1, 2]), &json!("1,2"), true);
            test_equality(&json!(0), &json!(null), false);
        }
    }

    // !=
    mod not_equal {
        use super::*;

        fn test_not_equal(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "!=": [a, b] })), Ok(Value::Bool(expect)));
            assert_eq!(apply(&json!({ "!=": [b, a] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_not_equal() {
            assert_eq!(apply(&json!({ "!=": [] })), Ok(Value::Bool(false)));
            assert_eq!(apply(&json!({ "!=": Value::Null })), Ok(Value::Bool(false)));
            assert_eq!(
                apply(&json!({ "!=": [Value::Null] })),
                Ok(Value::Bool(false))
            );
            assert_eq!(apply(&json!({ "!=": "foo" })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!=": ["foo"] })), Ok(Value::Bool(true)));
            test_not_equal(&json!(null), &json!(null), false);
            test_not_equal(&json!(1), &json!(1), false);
            test_not_equal(&json!("foo"), &json!("foo"), false);

            assert_eq!(apply(&json!({ "!=": 0 })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!=": [0] })), Ok(Value::Bool(true)));
            test_not_equal(&json!(1), &json!(0), true);
        }

        #[test]
        fn not_equal_with_type_coercion() {
            test_not_equal(&json!(0), &json!(""), false);
            test_not_equal(&json!(1), &json!("1"), false);
            test_not_equal(&json!([1]), &json!("1"), false);
            test_not_equal(&json!([1, 2]), &json!("1,2"), false);
            test_not_equal(&json!(0), &json!(null), true);
        }
    }

    // ===
    mod strict_equality {
        use super::*;

        fn test_strict_equality(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "===": [a, b] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_strict_equality() {
            assert_eq!(apply(&json!({ "===": [] })), Ok(Value::Bool(true)));
            // For whatever reason the javascript implementation returns true for `null` instead of an
            // array as the argument.
            assert_eq!(apply(&json!({ "===": Value::Null })), Ok(Value::Bool(true)));
            assert_eq!(
                apply(&json!({ "===": [Value::Null] })),
                Ok(Value::Bool(true))
            );
            test_strict_equality(&json!(null), &json!(null), true);
            test_strict_equality(&json!(1), &json!(1), true);
            test_strict_equality(&json!("foo"), &json!("foo"), true);

            assert_eq!(apply(&json!({ "===": 0 })), Ok(Value::Bool(false)));
            assert_eq!(apply(&json!({ "===": [0] })), Ok(Value::Bool(false)));
            test_strict_equality(&json!(1), &json!(0), false);
        }

        #[test]
        fn strict_equality_with_type_coercion() {
            test_strict_equality(&json!(0), &json!(""), false);
            test_strict_equality(&json!(1), &json!("1"), false);
            test_strict_equality(&json!([1]), &json!("1"), false);
            test_strict_equality(&json!([1, 2]), &json!("1,2"), false);
            test_strict_equality(&json!(0), &json!(null), false);
        }
    }

    // !==
    mod strict_not_equal {
        use super::*;

        fn test_strict_not_equal(a: &Value, b: &Value, expect: bool) {
            assert_eq!(apply(&json!({ "!==": [a, b] })), Ok(Value::Bool(expect)));
        }

        #[test]
        fn simple_strict_equality() {
            assert_eq!(apply(&json!({ "!==": [] })), Ok(Value::Bool(false)));
            assert_eq!(
                apply(&json!({ "!==": Value::Null })),
                Ok(Value::Bool(false))
            );
            assert_eq!(
                apply(&json!({ "!==": [Value::Null] })),
                Ok(Value::Bool(false))
            );
            assert_eq!(apply(&json!({ "!==": "foo" })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!==": ["foo"] })), Ok(Value::Bool(true)));
            test_strict_not_equal(&json!(null), &json!(null), false);
            test_strict_not_equal(&json!(1), &json!(1), false);
            test_strict_not_equal(&json!("foo"), &json!("foo"), false);

            assert_eq!(apply(&json!({ "!==": 0 })), Ok(Value::Bool(true)));
            assert_eq!(apply(&json!({ "!==": [0] })), Ok(Value::Bool(true)));
            test_strict_not_equal(&json!(1), &json!(0), true);
        }

        #[test]
        fn strict_equality_with_type_coercion() {
            test_strict_not_equal(&json!(0), &json!(""), true);
            test_strict_not_equal(&json!(1), &json!("1"), true);
            test_strict_not_equal(&json!([1]), &json!("1"), true);
            test_strict_not_equal(&json!([1, 2]), &json!("1,2"), true);
            test_strict_not_equal(&json!(0), &json!(null), true);
        }
    }
}