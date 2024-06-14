use serde_json::{json, Value};

use super::{logic, Data, Expression, PartialResult};

/// Logical negation ("not"). Takes just one argument.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = args.get(0).map(|a| a.compute(data)).unwrap_or(json!(null));

    Value::Bool(!logic::is_truthy(&a))
}

// early returns on finding the arg as Ambiguous
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let a = args
        .get(0)
        .map(|a| a.partial_compute(data))
        .unwrap_or(Ok(Value::Null))?;

    Ok(Value::Bool(!logic::is_truthy(&a)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;

    #[test]
    fn test() {
        assert_eq!(compute_const!(), Value::Bool(true));
        assert_eq!(compute_const!(json!(null)), Value::Bool(true));
        assert_eq!(compute_const!(json!(false)), Value::Bool(true));
        assert_eq!(compute_const!(json!(true)), Value::Bool(false));
    }
}
