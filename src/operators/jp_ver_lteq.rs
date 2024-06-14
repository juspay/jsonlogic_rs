use super::jp_version::compare_version;
use super::{logic, Ambiguous, Data, Expression, PartialResult};

use serde_json::Value;

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let first = match args.get(0) {
        Some(arg) => logic::coerce_to_str(&arg.compute(data)),
        None => return Value::Bool(false),
    };

    let second = match args.get(1) {
        Some(arg) => logic::coerce_to_str(&arg.compute(data)),
        None => return Value::Bool(false),
    };

    let third = args
        .get(2)
        .map(|arg| logic::coerce_to_str(&arg.compute(data)));

    let result = compare_version(&first, &second, true).is_le()
        && third.map_or(true, |third| compare_version(&second, &third, true).is_le());

    Value::Bool(result)
}

// early returns false on obvious false conditions and returns Ambiguous on other Ambiguous cases
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let first = match args.get(0) {
        Some(arg) => arg.partial_compute(data).map(|a| logic::coerce_to_str(&a)),
        None => return Ok(Value::Bool(false)),
    };

    let second = match args.get(1) {
        Some(arg) => arg.partial_compute(data).map(|b| logic::coerce_to_str(&b)),
        None => return Ok(Value::Bool(false)),
    };

    let result = match args.get(2) {
        None => compare_version(&first?, &second?, true).is_le(),
        Some(third_arg) => {
            let third = third_arg
                .partial_compute(data)
                .map(|c| logic::coerce_to_str(&c));
            match (first, second, third) {
                (Ok(first), Ok(second), Ok(third)) => {
                    compare_version(&first, &second, true).is_le()
                        && compare_version(&second, &third, true).is_le()
                }
                (Ok(arg1), Err(Ambiguous), Ok(arg2))
                | (Err(Ambiguous), Ok(arg1), Ok(arg2))
                | (Ok(arg1), Ok(arg2), Err(Ambiguous))
                    if !compare_version(&arg1, &arg2, true).is_le() =>
                {
                    return Ok(Value::Bool(false))
                }
                _ => return Err(Ambiguous),
            }
        }
    };

    Ok(Value::Bool(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn less_equal_than() {
        assert_eq!(compute_const!(), Value::Bool(false));
        assert_eq!(compute_const!(json!("test")), Value::Bool(false));
        assert_eq!(compute_const!(json!("1.0.3")), Value::Bool(false));
        assert_eq!(
            compute_const!(json!("1.0.3"), json!("test")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("1.0.2"), json!("1.0.3")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("1.0.3"), json!("1.0.3")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("1.0.3"), json!("1.0.2")),
            Value::Bool(false)
        );
    }

    #[test]
    fn between_inclusive() {
        assert_eq!(
            compute_const!(json!("1.0.2"), json!("1.0.3"), json!("1.0.4")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("1.0.2"), json!("1.0.3"), json!("1.0.3")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("1.0.2"), json!("1.0.2"), json!("1.0.3")),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!("1.0.4"), json!("1.0.3"), json!("1.0.2")),
            Value::Bool(false)
        );
        assert_eq!(
            compute_const!(json!("1.0.2"), json!("1.0.1"), json!("1.0.3")),
            Value::Bool(false)
        );
    }
}
