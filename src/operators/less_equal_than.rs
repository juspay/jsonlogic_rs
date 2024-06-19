use serde_json::Value;

use super::{logic, Ambiguous, Data, Expression, PartialResult};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let result = match args.get(2) {
        Some(c) => compute_between_inclusive(&a, &b, &c.compute(data)),
        None => compute_less_equal_than(&a, &b),
    };

    Value::Bool(result)
}

// early returns false on obvious false conditions and returns Ambiguous on other Ambiguous cases
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let a = match args.get(0) {
        Some(arg) => arg.partial_compute(data),
        None => return Ok(Value::Bool(false)),
    };

    let b = match args.get(1) {
        Some(arg) => arg.partial_compute(data),
        None => return Ok(Value::Bool(false)),
    };

    let result = match args.get(2) {
        None => compute_less_equal_than(&a?, &b?),
        Some(third_arg) => {
            let c = third_arg.partial_compute(data);
            match (a, b, c) {
                (Ok(a), Ok(b), Ok(c)) => compute_between_inclusive(&a, &b, &c),
                (Ok(arg1), Err(Ambiguous), Ok(arg2))
                | (Err(Ambiguous), Ok(arg1), Ok(arg2))
                | (Ok(arg1), Ok(arg2), Err(Ambiguous))
                    if !compute_less_equal_than(&arg1, &arg2) =>
                {
                    return Ok(Value::Bool(false))
                }
                _ => return Err(Ambiguous),
            }
        }
    };

    Ok(Value::Bool(result))
}

fn compute_less_equal_than(a: &Value, b: &Value) -> bool {
    logic::less_equal_than(a, b)
}

/// Checks whether the value `b` is between `a` and `c`, including the bounds.
fn compute_between_inclusive(a: &Value, b: &Value, c: &Value) -> bool {
    logic::less_equal_than(a, b) && logic::less_equal_than(b, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn less_equal_than() {
        assert_eq!(compute_const!(), Value::Bool(false));
        assert_eq!(compute_const!(json!(1)), Value::Bool(false));
        assert_eq!(compute_const!(json!(1), json!(2)), Value::Bool(true));
        assert_eq!(compute_const!(json!(2), json!(2)), Value::Bool(true));
        assert_eq!(compute_const!(json!(3), json!(2)), Value::Bool(false));
    }

    #[test]
    fn between_inclusive() {
        assert_eq!(
            compute_const!(json!(1), json!(2), json!(3)),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!(1), json!(2), json!(2)),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!(2), json!(2), json!(3)),
            Value::Bool(true)
        );
        assert_eq!(
            compute_const!(json!(2), json!(4), json!(3)),
            Value::Bool(false)
        );
    }
}
