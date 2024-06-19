use serde_json::{json, Value};

use super::{logic, Data, Expression, PartialResult};

pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = args
        .get(0)
        .map(|arg| arg.compute(data))
        .unwrap_or(json!(null));
    let b = args
        .get(1)
        .map(|arg| arg.compute(data))
        .unwrap_or(json!(null));

    Value::Bool(logic::is_strict_equal(&a, &b))
}

// early returns on finding either Ambiguous arg
pub fn partial_compute(args: &[Expression], data: &Data) -> PartialResult {
    let a = args
        .get(0)
        .map(|arg| arg.partial_compute(data))
        .unwrap_or(Ok(Value::Null))?;
    let b = args
        .get(1)
        .map(|arg| arg.partial_compute(data))
        .unwrap_or(Ok(Value::Null))?;

    Ok(Value::Bool(logic::is_strict_equal(&a, &b)))
}
