use serde_json::{Number, Value};

use super::{logic, Data, Expression, PartialResult};

/// "/", takes two arguments that are coerced into numbers. Returns `Value::Null` if the divisor is
/// coerced to `0` or one argument cannot be coerced into a number.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args
        .get(0)
        .map(|arg| arg.compute(data))
        .and_then(|a| logic::coerce_to_f64(&a))
    {
        Some(a) => a,
        None => return Value::Null,
    };

    let b = match args
        .get(1)
        .map(|arg| arg.compute(data))
        .and_then(|b| logic::coerce_to_f64(&b))
    {
        Some(b) => b,
        None => return Value::Null,
    };

    match Number::from_f64(a / b) {
        Some(num) => Value::Number(num),
        None => Value::Null,
    }
}

// early returns on finding either Ambiguous arg
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let a = match args
        .get(0)
        .map(|arg| arg.partial_compute(data))
        .transpose()?
        .and_then(|a| logic::coerce_to_f64(&a))
    {
        Some(a) => a,
        None => return Ok(Value::Null),
    };

    let b = match args
        .get(1)
        .map(|arg| arg.partial_compute(data))
        .transpose()?
        .and_then(|b| logic::coerce_to_f64(&b))
    {
        Some(b) => b,
        None => return Ok(Value::Null),
    };

    match Number::from_f64(a / b) {
        Some(num) => Ok(Value::Number(num)),
        None => Ok(Value::Null),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

    #[test]
    fn null() {
        assert_eq!(compute_const!(), Value::Null);
        assert_eq!(compute_const!(json!("a")), Value::Null);
        assert_eq!(compute_const!(json!(1)), Value::Null);
        assert_eq!(compute_const!(json!(1), json!(0)), Value::Null);

        assert_eq!(compute_const!(json!(1), json!(2)), json!(0.5));
    }
}
